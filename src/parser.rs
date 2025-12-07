//! DWARF parser implementation

use crate::error::Result;
use crate::types::*;
use gimli::{AttributeValue, DebuggingInformationEntry, Dwarf, Reader};
use object::{Object, ObjectSection, ObjectSymbol};
use std::borrow::Cow;
use std::collections::HashMap;

/// Apply relocations to a DWARF section
fn apply_relocations<'a>(
    object_file: &object::File,
    section_data: &'a [u8],
    section_name: &str,
) -> Cow<'a, [u8]> {
    use object::RelocationTarget;

    // Get the DWARF section to access its relocations
    let dwarf_section = match object_file.section_by_name(section_name) {
        Some(section) => section,
        None => return Cow::Borrowed(section_data),
    };

    // Clone the section data so we can modify it
    let mut data = section_data.to_vec();

    // Get relocations from the DWARF section
    for (offset, relocation) in dwarf_section.relocations() {
        let offset = offset as usize;

        // Get the value to add (from the symbol or addend)
        let value: u64 = match relocation.target() {
            RelocationTarget::Symbol(symbol_idx) => {
                if let Ok(symbol) = object_file.symbol_by_index(symbol_idx) {
                    // For section symbols, use the section's address (0 for object files)
                    // plus the addend
                    if symbol.kind() == object::SymbolKind::Section {
                        relocation.addend() as u64
                    } else {
                        symbol.address().wrapping_add(relocation.addend() as u64)
                    }
                } else {
                    relocation.addend() as u64
                }
            }
            _ => relocation.addend() as u64,
        };

        // Apply the relocation based on its type
        use object::RelocationKind;
        match relocation.kind() {
            RelocationKind::Absolute if relocation.size() == 32 => {
                // R_X86_64_32: S + A (32-bit absolute)
                if offset + 4 <= data.len() {
                    let bytes = (value as u32).to_le_bytes();
                    data[offset..offset + 4].copy_from_slice(&bytes);
                }
            }
            RelocationKind::Absolute if relocation.size() == 64 => {
                // R_X86_64_64: S + A (64-bit absolute)
                if offset + 8 <= data.len() {
                    let bytes = value.to_le_bytes();
                    data[offset..offset + 8].copy_from_slice(&bytes);
                }
            }
            _ => {
                // Ignore other relocation types
            }
        }
    }

    Cow::Owned(data)
}

pub struct DwarfParser<'a> {
    dwarf: Dwarf<DwarfReader<'a>>,
    type_cache: HashMap<usize, TypeInfo>,
    typedef_map: HashMap<usize, TypedefInfo>,
    abstract_origins: HashMap<usize, String>,
    // Keep the relocated section data alive in stable heap allocations
    _section_data: Vec<Box<[u8]>>,
}
#[allow(dead_code)] // Some parser methods are called via offset-based parsing
#[allow(clippy::too_many_arguments)] // Parser methods need many parameters from DWARF
#[allow(clippy::while_let_loop)] // Some loops are clearer with explicit match
impl<'a> DwarfParser<'a> {
    pub fn new(file_data: &'a [u8]) -> Result<Self> {
        let object = object::File::parse(file_data)?;

        // Pre-load and relocate all sections, storing them in stable heap allocations
        let mut section_data_map: HashMap<&str, Box<[u8]>> = HashMap::new();

        let section_ids = [
            gimli::SectionId::DebugAbbrev,
            gimli::SectionId::DebugInfo,
            gimli::SectionId::DebugLine,
            gimli::SectionId::DebugStr,
            gimli::SectionId::DebugStrOffsets,
            gimli::SectionId::DebugTypes,
            gimli::SectionId::DebugLoc,
            gimli::SectionId::DebugLocLists,
            gimli::SectionId::DebugRanges,
            gimli::SectionId::DebugRngLists,
            gimli::SectionId::DebugAddr,
            gimli::SectionId::DebugLineStr,
        ];

        // Preload all sections that need relocation, storing in stable Box allocations
        for &id in &section_ids {
            let section_data = object
                .section_by_name(id.name())
                .and_then(|section| section.data().ok())
                .unwrap_or(&[]);

            let relocated_data = apply_relocations(&object, section_data, id.name());
            if let Cow::Owned(data) = relocated_data {
                section_data_map.insert(id.name(), data.into_boxed_slice());
            }
        }

        // Now load sections with proper references
        // Box ensures the data address is stable even when moved
        let load_section =
            |id: gimli::SectionId| -> std::result::Result<DwarfReader<'a>, gimli::Error> {
                let section_data = object
                    .section_by_name(id.name())
                    .and_then(|section| section.data().ok())
                    .unwrap_or(&[]);

                let relocated_data = apply_relocations(&object, section_data, id.name());

                let data_ref: &'a [u8] = match relocated_data {
                    Cow::Borrowed(data) => data,
                    Cow::Owned(_) => {
                        // Use the pre-stored relocated data from the map
                        // SAFETY: We're extending the lifetime of the reference from the Box to 'a.
                        // This is safe because:
                        // 1. section_data_map contains the relocated data in Box (stable address)
                        // 2. The Box data will be moved into the DwarfParser struct
                        // 3. The DwarfParser has lifetime 'a
                        // 4. The references won't outlive the parser
                        if let Some(boxed_data) = section_data_map.get(id.name()) {
                            unsafe {
                                std::slice::from_raw_parts(boxed_data.as_ptr(), boxed_data.len())
                            }
                        } else {
                            &[]
                        }
                    }
                };

                Ok(gimli::EndianSlice::new(data_ref, gimli::LittleEndian))
            };

        let dwarf = Dwarf::load(load_section)?;

        // Convert HashMap values to Vec for storage
        let section_data_storage: Vec<Box<[u8]>> = section_data_map.into_values().collect();

        Ok(DwarfParser {
            dwarf,
            type_cache: HashMap::new(),
            typedef_map: HashMap::new(),
            abstract_origins: HashMap::new(),
            _section_data: section_data_storage,
        })
    }

    pub fn parse(&mut self) -> Result<Vec<CompileUnit>> {
        let mut compile_units = Vec::new();

        // First pass: collect typedefs and abstract origins
        let mut units = self.dwarf.units();
        while let Some(header) = units.next()? {
            let unit = self.dwarf.unit(header)?;
            self.collect_metadata(&unit)?;
        }

        // Second pass: parse compile units
        let mut units = self.dwarf.units();
        while let Some(header) = units.next()? {
            let unit = self.dwarf.unit(header)?;
            if let Some(mut cu) = self.parse_compile_unit(&unit)? {
                // First do intra-CU matching (uses decl_offset/spec_offset)
                Self::match_method_definitions(&mut cu.elements);
                compile_units.push(cu);
            }
        }

        // Third pass: cross-CU matching for methods that couldn't be matched within their CU
        // This handles the case where class declarations are in headers included by multiple CUs,
        // but method definitions are only in one CU
        Self::cross_cu_match_method_definitions(&mut compile_units);

        Ok(compile_units)
    }

    /// Match method declarations across all CUs using linkage names
    /// This handles cases where a header is included in multiple CUs but the method
    /// definitions are only in one CU (the .cpp file)
    fn cross_cu_match_method_definitions(compile_units: &mut [CompileUnit]) {
        use std::collections::HashMap;

        fn collect_definitions(
            elements: &[Element],
            definitions: &mut HashMap<String, MethodDefinition>,
        ) {
            for element in elements {
                match element {
                    Element::Function(func) => {
                        // Only index method definitions (those with specification_offset, meaning
                        // they reference a class declaration)
                        if func.is_method && func.specification_offset.is_some() {
                            if let Some(ref linkage_name) = func.linkage_name {
                                definitions
                                    .entry(linkage_name.clone())
                                    .or_insert_with(|| MethodDefinition::from_function(func));
                            }
                        }
                    }
                    Element::Namespace(ns) => {
                        collect_definitions(&ns.children, definitions);
                    }
                    _ => {}
                }
            }
        }

        fn apply_matches(
            elements: &mut [Element],
            definitions: &HashMap<String, MethodDefinition>,
        ) {
            for element in elements.iter_mut() {
                match element {
                    Element::Compound(compound) => {
                        for method in &mut compound.methods {
                            // Only try cross-CU matching if method has no parameters yet
                            // (meaning intra-CU matching didn't find a definition)
                            if method.parameters.is_empty() {
                                if let Some(ref linkage_name) = method.linkage_name {
                                    if let Some(def) = definitions.get(linkage_name) {
                                        def.apply_to_method(method);
                                    }
                                }
                            }
                        }
                    }
                    Element::Namespace(ns) => {
                        apply_matches(&mut ns.children, definitions);
                    }
                    _ => {}
                }
            }
        }

        // First, collect all method definitions by linkage name from all CUs
        let mut definitions_by_linkage: HashMap<String, MethodDefinition> = HashMap::new();

        for cu in compile_units.iter() {
            collect_definitions(&cu.elements, &mut definitions_by_linkage);
        }

        // Then, match unmatched method declarations using the global map
        for cu in compile_units.iter_mut() {
            apply_matches(&mut cu.elements, &definitions_by_linkage);
        }
    }

    /// Match method declarations in classes with their definitions at top level
    fn match_method_definitions(elements: &mut [Element]) {
        use std::collections::HashMap;

        // Build maps of definitions:
        // 1. By linkage name (for methods that have it on both declaration and definition)
        // 2. By specification offset (for methods that use DW_AT_specification)
        let mut definitions_by_linkage: HashMap<String, MethodDefinition> = HashMap::new();
        let mut definitions_by_spec_offset: HashMap<usize, MethodDefinition> = HashMap::new();

        for element in elements.iter() {
            if let Element::Function(func) = element {
                let definition_data = MethodDefinition::from_function(func);

                // If this function has a specification_offset, it's a definition referencing a declaration
                if let Some(spec_offset) = func.specification_offset {
                    definitions_by_spec_offset.insert(spec_offset, definition_data.clone());
                }

                // Also index by linkage name for backwards compatibility
                if let Some(ref linkage_name) = func.linkage_name {
                    if !func.is_method || func.specification_offset.is_some() {
                        definitions_by_linkage.insert(linkage_name.clone(), definition_data);
                    }
                }
            }
        }

        // Match methods with definitions
        for element in elements.iter_mut() {
            match element {
                Element::Compound(compound) => {
                    for method in &mut compound.methods {
                        // First try to match by decl_offset (specification_offset on definition points to this)
                        let matched = if let Some(decl_offset) = method.decl_offset {
                            if let Some(def) = definitions_by_spec_offset.get(&decl_offset) {
                                def.apply_to_method(method);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        // Fall back to linkage name matching if specification matching didn't work
                        if !matched {
                            if let Some(ref linkage_name) = method.linkage_name {
                                if let Some(def) = definitions_by_linkage.get(linkage_name) {
                                    def.apply_to_method(method);
                                }
                            }
                        }
                    }
                }
                Element::Namespace(ns) => {
                    Self::match_method_definitions(&mut ns.children);
                }
                _ => {}
            }
        }

        // Build mapping from decl_offset to class name for marking top-level functions
        let mut class_by_decl_offset: HashMap<usize, String> = HashMap::new();
        let mut class_by_linkage: HashMap<String, String> = HashMap::new();

        fn collect_class_methods(
            elements: &[Element],
            class_by_decl_offset: &mut HashMap<usize, String>,
            class_by_linkage: &mut HashMap<String, String>,
        ) {
            for element in elements.iter() {
                match element {
                    Element::Compound(compound) => {
                        if let Some(ref class_name) = compound.name {
                            for method in &compound.methods {
                                if let Some(decl_offset) = method.decl_offset {
                                    class_by_decl_offset.insert(decl_offset, class_name.clone());
                                }
                                if let Some(ref linkage_name) = method.linkage_name {
                                    class_by_linkage
                                        .insert(linkage_name.clone(), class_name.clone());
                                }
                            }
                        }
                    }
                    Element::Namespace(ns) => {
                        collect_class_methods(&ns.children, class_by_decl_offset, class_by_linkage);
                    }
                    _ => {}
                }
            }
        }

        collect_class_methods(elements, &mut class_by_decl_offset, &mut class_by_linkage);

        // Mark top-level functions that are method definitions and set their class_name
        for element in elements.iter_mut() {
            if let Element::Function(func) = element {
                // If the function doesn't have a class_name yet, try to find it
                if func.class_name.is_none() {
                    // Try to match by specification_offset first, then by linkage_name
                    let class_name = func
                        .specification_offset
                        .and_then(|offset| class_by_decl_offset.get(&offset).cloned())
                        .or_else(|| {
                            func.linkage_name
                                .as_ref()
                                .and_then(|name| class_by_linkage.get(name).cloned())
                        });

                    if let Some(class_name) = class_name {
                        func.is_method = true;
                        func.class_name = Some(class_name);
                    }
                }
            }
        }
    }

    fn collect_metadata(&mut self, unit: &DwarfUnit) -> Result<()> {
        let mut entries = unit.entries();

        // Get unit base offset for converting to absolute offsets
        let unit_base = unit
            .header
            .offset()
            .as_debug_info_offset()
            .map(|o| o.0)
            .unwrap_or(0);

        while let Some((_, entry)) = entries.next_dfs()? {
            let offset = entry.offset().0;
            let abs_offset = unit_base + offset;

            // Collect typedefs
            if entry.tag() == gimli::DW_TAG_typedef {
                if let Some(name) = self.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
                    let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);
                    if let Some(type_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                        // Convert to absolute offset
                        let abs_type_offset = unit_base + type_offset;
                        self.typedef_map.insert(
                            abs_type_offset,
                            TypedefInfo {
                                name,
                                line,
                                decl_file,
                            },
                        );
                    }
                }
            }

            // Collect abstract origins (for inlined functions)
            if entry.tag() == gimli::DW_TAG_subprogram {
                if let Some(name) = self.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    self.abstract_origins.insert(abs_offset, name);
                }
            }
        }

        Ok(())
    }

    fn parse_compile_unit(&mut self, unit: &DwarfUnit) -> Result<Option<CompileUnit>> {
        let mut entries = unit.entries();

        if let Some((_, entry)) = entries.next_dfs()? {
            if entry.tag() == gimli::DW_TAG_compile_unit {
                let name = self
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "unknown".to_string());
                let producer = self.get_string_attr(unit, entry, gimli::DW_AT_producer);

                // Extract file table from line program
                let file_table = self.extract_file_table(unit, entry)?;

                let mut elements = Vec::new();
                self.parse_children(unit, &mut entries, &mut elements)?;

                return Ok(Some(CompileUnit {
                    name,
                    producer,
                    elements,
                    file_table,
                }));
            }
        }

        Ok(None)
    }

    /// Extract the file table from the DWARF line program
    fn extract_file_table(
        &self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Vec<String>> {
        let mut file_table = Vec::new();

        // Try to get the line program from DW_AT_stmt_list
        if let Some(AttributeValue::DebugLineRef(line_offset)) = entry
            .attr(gimli::DW_AT_stmt_list)
            .ok()
            .flatten()
            .map(|a| a.value())
        {
            if let Ok(program) = self.dwarf.debug_line.program(
                line_offset,
                unit.header.address_size(),
                unit.comp_dir,
                unit.name,
            ) {
                let header = program.header();

                // File index 0 is defined to be the compile unit file (name from DW_AT_name)
                // but it's not always in the file table, so we don't add it here

                // Iterate through files in the file table (starting from index 1)
                for file_index in 1.. {
                    if let Some(file_entry) = header.file(file_index) {
                        // Get the file name
                        let mut path_buf = String::new();

                        // Get directory if present
                        if let Some(dir_attr) = file_entry.directory(header) {
                            if let Ok(dir_slice) = self.dwarf.attr_string(unit, dir_attr) {
                                if let Ok(dir_cow) = dir_slice.to_slice() {
                                    let dir_str = String::from_utf8_lossy(&dir_cow);
                                    if !dir_str.is_empty() {
                                        path_buf.push_str(&dir_str);
                                        path_buf.push('/');
                                    }
                                }
                            }
                        }

                        // Get file name
                        if let Ok(file_slice) = self.dwarf.attr_string(unit, file_entry.path_name())
                        {
                            if let Ok(file_cow) = file_slice.to_slice() {
                                let file_str = String::from_utf8_lossy(&file_cow);
                                path_buf.push_str(&file_str);
                            }
                        }

                        if !path_buf.is_empty() {
                            file_table.push(path_buf);
                        }
                    } else {
                        break; // No more files
                    }
                }
            }
        }

        Ok(file_table)
    }

    #[allow(unused_variables)] // Statistics tracking variables for debugging
    fn parse_children(
        &mut self,
        unit: &DwarfUnit,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
        elements: &mut Vec<Element>,
    ) -> Result<()> {
        let mut total_depth1 = 0;
        let mut captured = 0;
        let mut absolute_depth = 0; // Track absolute depth

        loop {
            let (depth_delta, entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            // Update absolute depth based on delta
            absolute_depth += depth_delta;

            // If we've gone back to compile unit level or beyond, we're done
            if absolute_depth <= 0 {
                break;
            }

            // Only process direct children of compile unit (absolute depth == 1)
            if absolute_depth == 1 {
                total_depth1 += 1;
                let tag = entry.tag();
                let offset = entry.offset();

                let captured_before = elements.len();

                match tag {
                    gimli::DW_TAG_namespace => {
                        if let Some(ns) = self.parse_namespace_at(unit, offset)? {
                            elements.push(Element::Namespace(ns));
                        }
                    }
                    gimli::DW_TAG_structure_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "struct")? {
                            elements.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_class_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "class")? {
                            elements.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_union_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "union")? {
                            elements.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_enumeration_type => {
                        if let Some(compound) = self.parse_enum_at(unit, offset)? {
                            elements.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_subprogram => {
                        if let Some(func) = self.parse_function_at(unit, offset, false)? {
                            elements.push(Element::Function(func));
                        }
                    }
                    gimli::DW_TAG_variable => {
                        if let Some(var) = self.parse_variable(unit, entry)? {
                            elements.push(Element::Variable(var));
                        }
                    }
                    gimli::DW_TAG_typedef => {
                        // Parse typedefs that point to other typedefs or base types
                        if let Some(typedef_alias) = self.parse_typedef_alias(unit, entry)? {
                            elements.push(Element::TypedefAlias(typedef_alias));
                        }
                    }
                    // Skip base_type at top level - they'll be resolved when referenced
                    gimli::DW_TAG_base_type
                    | gimli::DW_TAG_pointer_type
                    | gimli::DW_TAG_const_type
                    | gimli::DW_TAG_array_type
                    | gimli::DW_TAG_subroutine_type => {
                        // These are type definitions that we resolve on-demand
                    }
                    _ => {
                        // Unhandled tag
                    }
                }

                if elements.len() > captured_before {
                    captured += 1;
                }
            }
        }

        Ok(())
    }

    fn parse_namespace_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Namespace>> {
        // Create cursor at offset and parse
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        // Extract data from entry before recursive calls
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_namespace_children(unit, name, line, &mut entries)
    }

    fn parse_compound_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        compound_type: &str,
    ) -> Result<Option<Compound>> {
        self.parse_compound_offset(unit, offset, compound_type)
    }

    fn parse_enum_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Compound>> {
        self.parse_enum_offset(unit, offset)
    }

    fn parse_function_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        is_method: bool,
    ) -> Result<Option<Function>> {
        self.parse_function_offset(unit, offset, is_method)
    }

    fn parse_lexical_block_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<LexicalBlock>> {
        self.parse_lexical_block_offset(unit, offset)
    }

    fn parse_namespace_children(
        &mut self,
        unit: &DwarfUnit,
        name: String,
        line: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Namespace>> {
        let mut children = Vec::new();
        let mut absolute_depth = 1; // We start at the namespace level (depth 1 from compile unit)

        // Parse namespace children
        loop {
            let (depth_delta, child_entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            absolute_depth += depth_delta;

            if absolute_depth <= 1 {
                break;
            }

            if absolute_depth == 2 {
                let tag = child_entry.tag();
                let offset = child_entry.offset();

                match tag {
                    gimli::DW_TAG_namespace => {
                        if let Some(ns) = self.parse_namespace_at(unit, offset)? {
                            children.push(Element::Namespace(ns));
                        }
                    }
                    gimli::DW_TAG_structure_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "struct")? {
                            children.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_class_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "class")? {
                            children.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_union_type => {
                        if let Some(compound) = self.parse_compound_at(unit, offset, "union")? {
                            children.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_enumeration_type => {
                        if let Some(compound) = self.parse_enum_at(unit, offset)? {
                            children.push(Element::Compound(compound));
                        }
                    }
                    gimli::DW_TAG_subprogram => {
                        if let Some(func) = self.parse_function_at(unit, offset, false)? {
                            children.push(Element::Function(func));
                        }
                    }
                    gimli::DW_TAG_variable => {
                        if let Some(var) = self.parse_variable(unit, child_entry)? {
                            children.push(Element::Variable(var));
                        }
                    }
                    gimli::DW_TAG_typedef => {
                        if let Some(typedef_alias) = self.parse_typedef_alias(unit, child_entry)? {
                            children.push(Element::TypedefAlias(typedef_alias));
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(Namespace {
            name,
            line,
            children,
        }))
    }

    fn parse_compound_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        compound_type: &str,
    ) -> Result<Option<Compound>> {
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Convert to absolute offset for typedef lookup
        let offset_val = entry.offset().0;
        let unit_base = unit
            .header
            .offset()
            .as_debug_info_offset()
            .map(|o| o.0)
            .unwrap_or(0);
        let abs_offset = unit_base + offset_val;

        // Only merge typedef if it's in the same file as the struct
        let metadata = self.build_compound_metadata_with_typedef(
            name,
            line,
            byte_size,
            compound_type,
            decl_file,
            abs_offset,
        );

        self.parse_compound_children(unit, metadata, &mut entries)
    }

    fn parse_enum_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Compound>> {
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Convert to absolute offset for typedef lookup
        let offset_val = entry.offset().0;
        let unit_base = unit
            .header
            .offset()
            .as_debug_info_offset()
            .map(|o| o.0)
            .unwrap_or(0);
        let abs_offset = unit_base + offset_val;

        // Only merge typedef if it's in the same file as the enum
        let metadata = self.build_compound_metadata_with_typedef(
            name, line, byte_size, "enum", decl_file, abs_offset,
        );

        self.parse_enum_children(unit, metadata, &mut entries)
    }

    fn parse_function_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        is_method: bool,
    ) -> Result<Option<Function>> {
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        // Get the unit base for absolute offset calculation
        let unit_base = unit
            .header
            .offset()
            .as_debug_info_offset()
            .map(|o| o.0)
            .unwrap_or(0);

        // Check for DW_AT_specification - this indicates a method definition
        // that refers back to a declaration inside a class
        let specification_offset = self.get_ref_attr(unit, entry, gimli::DW_AT_specification);

        let (
            name,
            return_type,
            accessibility,
            is_virtual_from_spec,
            linkage_name_from_spec,
            spec_abs_offset,
        ) = if let Some(spec_offset) = specification_offset {
            // Follow the specification to get name, return_type, etc from the declaration
            let spec_unit_offset = gimli::UnitOffset(spec_offset);
            let mut spec_entries = unit.entries_at_offset(spec_unit_offset)?;

            if let Some((_, spec_entry)) = spec_entries.next_dfs()? {
                let name = match self.get_string_attr(unit, spec_entry, gimli::DW_AT_name) {
                    Some(n) => n,
                    None => return Ok(None),
                };
                let return_type = self.resolve_type(unit, spec_entry)?;
                let accessibility = self.get_accessibility(spec_entry);
                let is_virtual = self.get_bool_attr(spec_entry, gimli::DW_AT_virtuality);
                let linkage_name = self
                    .get_string_attr(unit, spec_entry, gimli::DW_AT_linkage_name)
                    .or_else(|| {
                        self.get_string_attr(unit, spec_entry, gimli::DW_AT_MIPS_linkage_name)
                    });
                let abs_offset = unit_base + spec_offset;
                (
                    name,
                    return_type,
                    accessibility,
                    is_virtual,
                    linkage_name,
                    Some(abs_offset),
                )
            } else {
                return Ok(None);
            }
        } else {
            // No specification - get name directly from entry
            let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
                Some(n) => n,
                None => return Ok(None),
            };

            let is_declaration = self.get_bool_attr(entry, gimli::DW_AT_declaration);
            if is_declaration && !is_method {
                return Ok(None);
            }

            let return_type = self.resolve_type(unit, entry)?;
            let accessibility = self.get_accessibility(entry);
            let is_virtual = self.get_bool_attr(entry, gimli::DW_AT_virtuality);
            let linkage_name = self
                .get_string_attr(unit, entry, gimli::DW_AT_linkage_name)
                .or_else(|| self.get_string_attr(unit, entry, gimli::DW_AT_MIPS_linkage_name));
            (
                name,
                return_type,
                accessibility,
                is_virtual,
                linkage_name,
                None,
            )
        };

        let is_declaration = self.get_bool_attr(entry, gimli::DW_AT_declaration);
        // If we have a specification, this is a definition (has body)
        let has_body = specification_offset.is_some() || !is_declaration;

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let low_pc = self.get_u64_attr(entry, gimli::DW_AT_low_pc);
        let mut high_pc = self.get_u64_attr(entry, gimli::DW_AT_high_pc);

        // high_pc can be either an absolute address or an offset from low_pc
        // If we have both and high_pc is small (likely an offset), convert it
        if let (Some(low), Some(high)) = (low_pc, high_pc) {
            if high < low {
                // high_pc is an offset, convert to absolute address
                high_pc = Some(low + high);
            }
        }

        let is_inline = self.get_u64_attr(entry, gimli::DW_AT_inline).is_some();
        let is_external = self.get_bool_attr(entry, gimli::DW_AT_external);
        // Use virtual flag from specification if we have one, otherwise from entry
        let is_virtual = is_virtual_from_spec || self.get_bool_attr(entry, gimli::DW_AT_virtuality);
        // Prefer linkage name from entry, fall back to one from specification
        let linkage_name = self
            .get_string_attr(unit, entry, gimli::DW_AT_linkage_name)
            .or_else(|| self.get_string_attr(unit, entry, gimli::DW_AT_MIPS_linkage_name))
            .or(linkage_name_from_spec);
        let is_artificial = self.get_bool_attr(entry, gimli::DW_AT_artificial);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Destructor detection by name (~ prefix)
        let is_destructor = name.starts_with('~');
        // Constructor detection will happen during generation when we have class name
        let is_constructor = false;

        // If we have a specification, this is a method definition
        let effective_is_method = is_method || spec_abs_offset.is_some();

        // For method declarations (inside classes), store their absolute offset so definitions can reference them
        let decl_offset = if is_method && is_declaration && specification_offset.is_none() {
            Some(unit_base + offset.0)
        } else {
            None
        };

        let metadata = FunctionMetadata {
            name,
            decl_file,
            line,
            return_type,
            accessibility,
            has_body,
            is_method: effective_is_method,
            low_pc,
            high_pc,
            is_inline,
            is_external,
            is_virtual,
            is_constructor,
            is_destructor,
            linkage_name,
            is_artificial,
            specification_offset: spec_abs_offset,
            decl_offset,
        };

        self.parse_function_children(unit, metadata, &mut entries)
    }

    fn parse_lexical_block_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<LexicalBlock>> {
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_lexical_block_children(unit, line, &mut entries)
    }

    fn parse_compound(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
        compound_type: &str,
    ) -> Result<Option<Compound>> {
        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Check if typedef - only merge if in same file
        let offset = entry.offset().0;
        let metadata = self.build_compound_metadata_with_typedef(
            name,
            line,
            byte_size,
            compound_type,
            decl_file,
            offset,
        );

        self.parse_compound_children(unit, metadata, entries)
    }

    /// Build CompoundMetadata with typedef information if applicable.
    /// Only merges typedef if it's in the same file as the compound type.
    fn build_compound_metadata_with_typedef(
        &self,
        name: Option<String>,
        line: Option<u64>,
        byte_size: Option<u64>,
        compound_type: &str,
        decl_file: Option<u64>,
        abs_offset: usize,
    ) -> CompoundMetadata {
        let (is_typedef, typedef_name, typedef_line) =
            if let Some(typedef_info) = self.typedef_map.get(&abs_offset) {
                // Only merge if BOTH have known file and they match
                // This prevents forward declaration typedefs from appearing in every file
                let same_file = match (decl_file, typedef_info.decl_file) {
                    (Some(a), Some(b)) => a == b,
                    _ => false, // If either is unknown, don't merge to avoid duplication
                };
                if same_file {
                    (true, Some(typedef_info.name.clone()), typedef_info.line)
                } else {
                    (false, None, None)
                }
            } else {
                (false, None, None)
            };

        CompoundMetadata {
            name,
            line,
            byte_size,
            is_typedef,
            typedef_name,
            typedef_line,
            compound_type: compound_type.to_string(),
            decl_file,
        }
    }

    fn parse_compound_children(
        &mut self,
        unit: &DwarfUnit,
        metadata: CompoundMetadata,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>> {
        let mut members = Vec::new();
        let mut methods = Vec::new();
        let mut base_classes = Vec::new();
        let mut nested_types = Vec::new();
        let mut is_virtual = false;
        let mut absolute_depth = 1; // We start at the struct/union level (depth 1 from compile unit)

        // Parse members
        loop {
            let (depth_delta, child_entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            absolute_depth += depth_delta;

            if absolute_depth <= 1 {
                break;
            }

            if absolute_depth == 2 {
                let tag = child_entry.tag();
                let offset = child_entry.offset();

                match tag {
                    gimli::DW_TAG_member => {
                        if let Some(var) = self.parse_member(unit, child_entry)? {
                            // Check if this is a vtable pointer
                            if var.name.starts_with("_vptr") || var.name == "__vptr" {
                                is_virtual = true;
                            }
                            members.push(var);
                        }
                    }
                    gimli::DW_TAG_subprogram => {
                        // This is a method declaration
                        if let Some(mut func) = self.parse_function_at(unit, offset, true)? {
                            // Set the class name for constructor detection
                            func.class_name = metadata.name.clone();
                            methods.push(func);
                        }
                    }
                    gimli::DW_TAG_inheritance => {
                        if let Some(base) = self.parse_inheritance(unit, child_entry)? {
                            base_classes.push(base);
                        }
                    }
                    gimli::DW_TAG_structure_type => {
                        // Nested struct
                        if let Some(compound) = self.parse_compound_at(unit, offset, "struct")? {
                            nested_types.push(compound);
                        }
                    }
                    gimli::DW_TAG_class_type => {
                        // Nested class
                        if let Some(compound) = self.parse_compound_at(unit, offset, "class")? {
                            nested_types.push(compound);
                        }
                    }
                    gimli::DW_TAG_union_type => {
                        // Nested union
                        if let Some(compound) = self.parse_compound_at(unit, offset, "union")? {
                            nested_types.push(compound);
                        }
                    }
                    gimli::DW_TAG_enumeration_type => {
                        // Nested enum
                        if let Some(compound) = self.parse_enum_at(unit, offset)? {
                            nested_types.push(compound);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(Compound {
            name: metadata.name,
            compound_type: metadata.compound_type,
            members,
            methods,
            nested_types,
            enum_values: Vec::new(),
            line: metadata.line,
            is_typedef: metadata.is_typedef,
            typedef_name: metadata.typedef_name,
            typedef_line: metadata.typedef_line,
            byte_size: metadata.byte_size,
            base_classes,
            is_virtual,
            decl_file: metadata.decl_file,
        }))
    }

    fn parse_enum(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>> {
        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Check if typedef - only merge if in same file
        let offset = entry.offset().0;
        let metadata = self
            .build_compound_metadata_with_typedef(name, line, byte_size, "enum", decl_file, offset);

        self.parse_enum_children(unit, metadata, entries)
    }

    fn parse_enum_children(
        &mut self,
        unit: &DwarfUnit,
        metadata: CompoundMetadata,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>> {
        let mut enum_values = Vec::new();
        let mut absolute_depth = 1; // We start at the enum level (depth 1 from compile unit)

        // Parse enumerators
        loop {
            let (depth_delta, child_entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            absolute_depth += depth_delta;

            if absolute_depth <= 1 {
                break;
            }

            if absolute_depth == 2 && child_entry.tag() == gimli::DW_TAG_enumerator {
                if let Some(enum_name) = self.get_string_attr(unit, child_entry, gimli::DW_AT_name)
                {
                    let value = self.get_i64_attr(child_entry, gimli::DW_AT_const_value);
                    enum_values.push((enum_name, value));
                }
            }
        }

        Ok(Some(Compound {
            name: metadata.name,
            compound_type: metadata.compound_type,
            members: Vec::new(),
            methods: Vec::new(),
            nested_types: Vec::new(),
            enum_values,
            line: metadata.line,
            is_typedef: metadata.is_typedef,
            typedef_name: metadata.typedef_name,
            typedef_line: metadata.typedef_line,
            byte_size: metadata.byte_size,
            base_classes: Vec::new(),
            decl_file: metadata.decl_file,
            is_virtual: false,
        }))
    }

    fn parse_member(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Variable>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let type_info = self.resolve_type(unit, entry)?;
        let accessibility = self.get_accessibility(entry);
        let offset = self.get_member_offset(unit, entry);
        let bit_size = self.get_u64_attr(entry, gimli::DW_AT_bit_size);
        let bit_offset = self
            .get_u64_attr(entry, gimli::DW_AT_bit_offset)
            .or_else(|| self.get_u64_attr(entry, gimli::DW_AT_data_bit_offset));

        let const_value = self.get_const_value(entry);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        Ok(Some(Variable {
            name,
            type_info,
            line,
            accessibility,
            offset,
            bit_size,
            bit_offset,
            const_value,
            decl_file,
        }))
    }

    fn parse_inheritance(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<BaseClass>> {
        // Get the type of the base class
        let type_info = self.resolve_type(unit, entry)?;
        let type_name = type_info.base_type;

        // Get offset of the base class within the derived class
        let offset = self.get_member_offset(unit, entry);

        // Get accessibility (public, protected, private)
        let accessibility = self.get_accessibility(entry);

        // Check if it's virtual inheritance
        let is_virtual = self.get_bool_attr(entry, gimli::DW_AT_virtuality);

        Ok(Some(BaseClass {
            type_name,
            offset,
            accessibility,
            is_virtual,
        }))
    }

    fn parse_variable(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Variable>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let mut type_info = self.resolve_type(unit, entry)?;

        // Check for extern/static
        if self.get_bool_attr(entry, gimli::DW_AT_external) {
            type_info.is_extern = true;
        }

        let const_value = self.get_const_value(entry);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        Ok(Some(Variable {
            name,
            type_info,
            line,
            accessibility: None,
            offset: None,
            bit_size: None,
            bit_offset: None,
            const_value,
            decl_file,
        }))
    }

    fn parse_function(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
        is_method: bool,
    ) -> Result<Option<Function>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        // Skip declarations unless they're method declarations
        let is_declaration = self.get_bool_attr(entry, gimli::DW_AT_declaration);
        if is_declaration && !is_method {
            return Ok(None);
        }

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let return_type = self.resolve_type(unit, entry)?;
        let accessibility = self.get_accessibility(entry);
        let has_body = !is_declaration;
        let low_pc = self.get_u64_attr(entry, gimli::DW_AT_low_pc);
        let mut high_pc = self.get_u64_attr(entry, gimli::DW_AT_high_pc);

        // high_pc can be either an absolute address or an offset from low_pc
        // If we have both and high_pc is small (likely an offset), convert it
        if let (Some(low), Some(high)) = (low_pc, high_pc) {
            if high < low {
                // high_pc is an offset, convert to absolute address
                high_pc = Some(low + high);
            }
        }

        let is_inline = self.get_u64_attr(entry, gimli::DW_AT_inline).is_some();
        let is_external = self.get_bool_attr(entry, gimli::DW_AT_external);
        let is_virtual = self.get_bool_attr(entry, gimli::DW_AT_virtuality);
        let linkage_name = self
            .get_string_attr(unit, entry, gimli::DW_AT_linkage_name)
            .or_else(|| self.get_string_attr(unit, entry, gimli::DW_AT_MIPS_linkage_name));
        let is_artificial = self.get_bool_attr(entry, gimli::DW_AT_artificial);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Destructor detection by name (~ prefix)
        let is_destructor = is_method && name.starts_with('~');
        // Constructor detection will happen during generation when we have class name
        let is_constructor = false;

        // Get the unit base for absolute offset calculation
        let unit_base = unit
            .header
            .offset()
            .as_debug_info_offset()
            .map(|o| o.0)
            .unwrap_or(0);

        // For method declarations, store their absolute offset so definitions can reference them
        let decl_offset = if is_method && is_declaration {
            Some(unit_base + entry.offset().0)
        } else {
            None
        };

        let metadata = FunctionMetadata {
            name,
            decl_file,
            line,
            return_type,
            accessibility,
            has_body,
            is_method,
            low_pc,
            high_pc,
            is_inline,
            is_external,
            is_virtual,
            is_constructor,
            is_destructor,
            linkage_name,
            is_artificial,
            specification_offset: None, // this function is for declarations, not definitions
            decl_offset,
        };

        self.parse_function_children(unit, metadata, entries)
    }

    fn parse_function_children(
        &mut self,
        unit: &DwarfUnit,
        metadata: FunctionMetadata,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Function>> {
        let mut parameters = Vec::new();
        let mut variables = Vec::new();
        let mut lexical_blocks = Vec::new();
        let mut inlined_calls = Vec::new();
        let mut labels = Vec::new();
        let mut absolute_depth = 1; // We start at the function level (depth 1 from compile unit)

        // Parse function children
        loop {
            let (depth_delta, child_entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            absolute_depth += depth_delta;

            if absolute_depth <= 1 {
                break;
            }

            if absolute_depth == 2 {
                let tag = child_entry.tag();
                let offset = child_entry.offset();

                match tag {
                    gimli::DW_TAG_formal_parameter => {
                        if let Some(param) = self.parse_parameter(unit, child_entry)? {
                            parameters.push(param);
                        }
                    }
                    gimli::DW_TAG_variable => {
                        if let Some(var) = self.parse_variable(unit, child_entry)? {
                            variables.push(var);
                        }
                    }
                    gimli::DW_TAG_lexical_block => {
                        if let Some(block) = self.parse_lexical_block_at(unit, offset)? {
                            lexical_blocks.push(block);
                        }
                    }
                    gimli::DW_TAG_inlined_subroutine => {
                        if let Some(inlined) = self.parse_inlined_subroutine(unit, child_entry)? {
                            inlined_calls.push(inlined);
                        }
                    }
                    gimli::DW_TAG_label => {
                        if let Some(label) = self.parse_label(unit, child_entry)? {
                            labels.push(label);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(Function {
            name: metadata.name,
            return_type: metadata.return_type,
            parameters,
            variables,
            lexical_blocks,
            inlined_calls,
            labels,
            line: metadata.line,
            is_method: metadata.is_method,
            class_name: None,
            accessibility: metadata.accessibility,
            has_body: metadata.has_body,
            low_pc: metadata.low_pc,
            high_pc: metadata.high_pc,
            is_inline: metadata.is_inline,
            is_external: metadata.is_external,
            is_virtual: metadata.is_virtual,
            is_constructor: metadata.is_constructor,
            is_destructor: metadata.is_destructor,
            linkage_name: metadata.linkage_name,
            is_artificial: metadata.is_artificial,
            decl_file: metadata.decl_file,
            specification_offset: metadata.specification_offset,
            decl_offset: metadata.decl_offset,
        }))
    }

    fn parse_parameter(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Parameter>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let type_info = self.resolve_type(unit, entry)?;

        Ok(Some(Parameter {
            name,
            type_info,
            line,
        }))
    }

    fn parse_lexical_block(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<LexicalBlock>> {
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_lexical_block_children(unit, line, entries)
    }

    fn parse_lexical_block_children(
        &mut self,
        unit: &DwarfUnit,
        line: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<LexicalBlock>> {
        let mut variables = Vec::new();
        let mut nested_blocks = Vec::new();
        let mut inlined_calls = Vec::new();
        let mut labels = Vec::new();
        let mut absolute_depth = 2; // We start at the lexical block level (depth 2 from compile unit)

        loop {
            let (depth_delta, child_entry) = match entries.next_dfs()? {
                Some(pair) => pair,
                None => break,
            };

            absolute_depth += depth_delta;

            if absolute_depth <= 2 {
                break;
            }

            if absolute_depth == 3 {
                let tag = child_entry.tag();
                let offset = child_entry.offset();

                match tag {
                    gimli::DW_TAG_variable => {
                        if let Some(var) = self.parse_variable(unit, child_entry)? {
                            variables.push(var);
                        }
                    }
                    gimli::DW_TAG_lexical_block => {
                        if let Some(block) = self.parse_lexical_block_at(unit, offset)? {
                            nested_blocks.push(block);
                        }
                    }
                    gimli::DW_TAG_inlined_subroutine => {
                        if let Some(inlined) = self.parse_inlined_subroutine(unit, child_entry)? {
                            inlined_calls.push(inlined);
                        }
                    }
                    gimli::DW_TAG_label => {
                        if let Some(label) = self.parse_label(unit, child_entry)? {
                            labels.push(label);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(LexicalBlock {
            variables,
            nested_blocks,
            inlined_calls,
            labels,
            line,
        }))
    }

    fn parse_inlined_subroutine(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<InlinedSubroutine>> {
        // Try to get name from abstract origin
        let name = if let Some(origin_offset) =
            self.get_ref_attr(unit, entry, gimli::DW_AT_abstract_origin)
        {
            // Convert to absolute offset
            let unit_base = unit
                .header
                .offset()
                .as_debug_info_offset()
                .map(|o| o.0)
                .unwrap_or(0);
            let abs_origin_offset = unit_base + origin_offset;
            self.abstract_origins.get(&abs_origin_offset).cloned()
        } else {
            None
        };

        let name = match name {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self
            .get_u64_attr(entry, gimli::DW_AT_call_line)
            .or_else(|| self.get_u64_attr(entry, gimli::DW_AT_decl_line));

        Ok(Some(InlinedSubroutine { name, line }))
    }

    fn parse_label(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Label>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        Ok(Some(Label { name, line }))
    }

    /// Parse a typedef entry and return a TypedefAlias if it points to another typedef or base type
    /// For struct/class/union/enum, only create TypedefAlias if the typedef is in a different file
    /// (same-file typedefs are handled via merging with the compound definition)
    fn parse_typedef_alias(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<TypedefAlias>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Get what this typedef points to
        let type_offset = match self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
            Some(offset) => offset,
            None => return Ok(None), // typedef with no type
        };

        // Resolve the target type
        let unit_offset = gimli::UnitOffset(type_offset);
        let mut entries = unit.entries_at_offset(unit_offset)?;

        if let Some((_, type_entry)) = entries.next_dfs()? {
            // Follow typedef chains to find the ultimate target type
            let mut current_offset = type_offset;
            let mut max_depth = 20; // Prevent infinite loops

            loop {
                let unit_offset = gimli::UnitOffset(current_offset);
                let mut current_entries = unit.entries_at_offset(unit_offset)?;

                if let Some((_, current_entry)) = current_entries.next_dfs()? {
                    match current_entry.tag() {
                        gimli::DW_TAG_typedef => {
                            // Follow the typedef chain
                            if let Some(next_offset) =
                                self.get_ref_attr(unit, current_entry, gimli::DW_AT_type)
                            {
                                current_offset = next_offset;
                                max_depth -= 1;
                                if max_depth == 0 {
                                    break;
                                }
                                continue;
                            }
                            break;
                        }
                        gimli::DW_TAG_structure_type
                        | gimli::DW_TAG_class_type
                        | gimli::DW_TAG_union_type
                        | gimli::DW_TAG_enumeration_type => {
                            // Found the ultimate struct/class/union/enum target
                            // Check if this is a forward declaration (only has DW_AT_declaration)
                            let is_forward_decl =
                                self.get_bool_attr(current_entry, gimli::DW_AT_declaration);

                            if is_forward_decl {
                                // This is a forward declaration - the actual struct definition
                                // is elsewhere, so we need to generate the TypedefAlias.
                                // The typedef_map merging will happen in the file where the
                                // actual struct is defined, not here.
                                break;
                            }

                            // This is an actual struct definition (not forward declaration)
                            // Get the target type's decl_file to check if merge will happen
                            let target_decl_file =
                                self.get_u64_attr(current_entry, gimli::DW_AT_decl_file);

                            // Check if typedef and target are in the same file (merge will happen)
                            let same_file = match (decl_file, target_decl_file) {
                                (Some(a), Some(b)) => a == b,
                                _ => true, // If either is unknown, assume same file (merge will happen)
                            };

                            if same_file {
                                // Same file - these are handled by the typedef_map merging, skip them
                                return Ok(None);
                            }
                            // Different files - generate the TypedefAlias since the merge
                            // happens in the struct's definition file, not here
                            break;
                        }
                        _ => break, // Other types (base types, pointers, etc.) - proceed normally
                    }
                } else {
                    break;
                }
            }

            // Resolve the type without typedef substitution to get the raw underlying type
            // (e.g., "struct tag_mpFace" instead of "mpFace")
            let target_type = self.resolve_type_entry_raw(unit, type_entry)?;

            // Note: We no longer skip typedefs to compound types here.
            // The loop above already handles deduplication by returning Ok(None)
            // when the typedef will be merged with the struct definition in the same file.
            // If we reach here, it means either:
            // 1. The typedef points to a forward declaration (struct defined elsewhere)
            // 2. The typedef is in a different file from the struct definition
            // In both cases, we need to generate the TypedefAlias.

            Ok(Some(TypedefAlias {
                name,
                target_type,
                line,
                decl_file,
            }))
        } else {
            Ok(None)
        }
    }

    fn resolve_type(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        let type_offset = match self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
            Some(offset) => offset,
            None => return Ok(TypeInfo::new("void".to_string())),
        };

        self.resolve_type_from_offset(unit, type_offset)
    }

    fn resolve_type_from_offset(&mut self, unit: &DwarfUnit, offset: usize) -> Result<TypeInfo> {
        // Convert unit-relative offset to absolute offset for caching
        let unit_start = unit.header.offset().as_debug_info_offset().unwrap().0;
        let absolute_offset = unit_start + offset;

        // Check cache using absolute offset
        if let Some(cached) = self.type_cache.get(&absolute_offset) {
            return Ok(cached.clone());
        }

        let unit_offset = gimli::UnitOffset(offset);
        let mut entries = unit.entries_at_offset(unit_offset)?;

        if let Some((_, type_entry)) = entries.next_dfs()? {
            let type_info = self.resolve_type_entry(unit, type_entry)?;
            // Cache using absolute offset
            self.type_cache.insert(absolute_offset, type_info.clone());
            return Ok(type_info);
        }

        Ok(TypeInfo::new("void".to_string()))
    }

    fn resolve_type_entry(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        self.resolve_type_entry_impl(unit, entry, true)
    }

    /// Resolve a type entry without typedef substitution for compound types.
    /// Used when we need the raw underlying type (e.g., for generating typedef aliases).
    fn resolve_type_entry_raw(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        self.resolve_type_entry_impl(unit, entry, false)
    }

    fn resolve_type_entry_impl(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        use_typedef_substitution: bool,
    ) -> Result<TypeInfo> {
        match entry.tag() {
            gimli::DW_TAG_base_type => {
                let name = self
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "void".to_string());
                Ok(TypeInfo::new(name))
            }
            gimli::DW_TAG_pointer_type => {
                if let Some(pointed_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, pointed_offset)?;
                    // Special case: pointer to subroutine is already a function pointer,
                    // don't increment pointer_count
                    if !type_info.is_function_pointer {
                        type_info.pointer_count += 1;
                    }
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.pointer_count = 1;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_array_type => {
                let base_offset = self
                    .get_ref_attr(unit, entry, gimli::DW_AT_type)
                    .unwrap_or(0);
                let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;

                // Get array dimensions from subrange children
                let mut entries = unit.entries_at_offset(entry.offset())?;
                entries.next_dfs()?; // Skip the array type itself
                let mut absolute_depth = 0;

                while let Some((depth_delta, child_entry)) = entries.next_dfs()? {
                    absolute_depth += depth_delta;
                    if absolute_depth <= 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_subrange_type {
                        let size = if let Some(count) =
                            self.get_u64_attr(child_entry, gimli::DW_AT_count)
                        {
                            count as usize
                        } else if let Some(upper) =
                            self.get_u64_attr(child_entry, gimli::DW_AT_upper_bound)
                        {
                            (upper + 1) as usize
                        } else {
                            0
                        };
                        type_info.array_sizes.push(size);
                    }
                }

                Ok(type_info)
            }
            gimli::DW_TAG_const_type => {
                if let Some(base_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_const = true;
                    Ok(type_info)
                } else {
                    Ok(TypeInfo::new("const void".to_string()))
                }
            }
            gimli::DW_TAG_volatile_type => {
                if let Some(base_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_volatile = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_volatile = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_restrict_type => {
                if let Some(base_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_restrict = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_restrict = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_reference_type => {
                if let Some(ref_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, ref_offset)?;
                    type_info.is_reference = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_reference = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_rvalue_reference_type => {
                if let Some(ref_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, ref_offset)?;
                    type_info.is_rvalue_reference = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_rvalue_reference = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_typedef => {
                let name = self
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "void".to_string());
                Ok(TypeInfo::new(name))
            }
            gimli::DW_TAG_structure_type
            | gimli::DW_TAG_class_type
            | gimli::DW_TAG_union_type
            | gimli::DW_TAG_enumeration_type => {
                let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);

                if let Some(n) = name {
                    let prefix = match entry.tag() {
                        gimli::DW_TAG_structure_type => "struct ",
                        gimli::DW_TAG_class_type => "class ",
                        gimli::DW_TAG_union_type => "union ",
                        gimli::DW_TAG_enumeration_type => "enum ",
                        _ => "",
                    };

                    // Check if it has a typedef (only if substitution is enabled)
                    if use_typedef_substitution {
                        let offset = entry.offset().0;
                        if let Some(typedef_info) = self.typedef_map.get(&offset) {
                            return Ok(TypeInfo::new(typedef_info.name.clone()));
                        }
                    }
                    Ok(TypeInfo::new(format!("{}{}", prefix, n)))
                } else {
                    // Anonymous type, check for typedef (only if substitution is enabled)
                    if use_typedef_substitution {
                        let offset = entry.offset().0;
                        if let Some(typedef_info) = self.typedef_map.get(&offset) {
                            return Ok(TypeInfo::new(typedef_info.name.clone()));
                        }
                    }
                    Ok(TypeInfo::new("void".to_string()))
                }
            }
            gimli::DW_TAG_subroutine_type => {
                // Function pointer
                let mut func_type = TypeInfo::new("void".to_string());
                func_type.is_function_pointer = true;

                // Get return type
                if let Some(ret_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    if let Ok(ret_type) = self.resolve_type_from_offset(unit, ret_offset) {
                        func_type.function_return_type = Some(Box::new(ret_type));
                    }
                }

                // Get parameters
                let mut entries = unit.entries_at_offset(entry.offset())?;
                entries.next_dfs()?; // Skip the subroutine type itself
                let mut absolute_depth = 0;

                while let Some((depth_delta, child_entry)) = entries.next_dfs()? {
                    absolute_depth += depth_delta;
                    if absolute_depth <= 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_formal_parameter {
                        // Try to get the parameter type
                        if let Some(param_offset) =
                            self.get_ref_attr(unit, child_entry, gimli::DW_AT_type)
                        {
                            if let Ok(param_type) =
                                self.resolve_type_from_offset(unit, param_offset)
                            {
                                func_type.function_params.push(param_type);
                            } else {
                                // Cross-unit or unresolvable reference - use void* as fallback
                                let mut void_ptr = TypeInfo::new("void".to_string());
                                void_ptr.pointer_count = 1;
                                func_type.function_params.push(void_ptr);
                            }
                        } else {
                            // No type attribute or unresolvable - check if there's a name
                            // For unnamed parameters with no type, assume void* if formal_parameter exists
                            let mut void_ptr = TypeInfo::new("void".to_string());
                            void_ptr.pointer_count = 1;
                            func_type.function_params.push(void_ptr);
                        }
                    }
                }

                Ok(func_type)
            }
            _ => Ok(TypeInfo::new("void".to_string())),
        }
    }

    fn get_string_attr(
        &self,
        _unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<String> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::String(s) => {
                    if let Ok(slice) = s.to_slice() {
                        return Some(String::from_utf8_lossy(&slice).to_string());
                    }
                }
                AttributeValue::DebugStrRef(offset) => {
                    if let Ok(s) = self.dwarf.debug_str.get_str(offset) {
                        if let Ok(slice) = s.to_slice() {
                            return Some(String::from_utf8_lossy(&slice).to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn get_u64_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<u64> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::Udata(v) => return Some(v),
                AttributeValue::Data1(v) => return Some(v as u64),
                AttributeValue::Data2(v) => return Some(v as u64),
                AttributeValue::Data4(v) => return Some(v as u64),
                AttributeValue::Data8(v) => return Some(v),
                AttributeValue::Addr(v) => return Some(v),
                AttributeValue::FileIndex(v) => return Some(v),
                _ => {}
            }
        }
        None
    }

    fn get_i64_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<i64> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::Sdata(v) => return Some(v),
                AttributeValue::Udata(v) => return Some(v as i64),
                AttributeValue::Data1(v) => return Some(v as i64),
                AttributeValue::Data2(v) => return Some(v as i64),
                AttributeValue::Data4(v) => return Some(v as i64),
                AttributeValue::Data8(v) => return Some(v as i64),
                _ => {}
            }
        }
        None
    }

    fn get_bool_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> bool {
        if let Some(attr_value) = entry.attr(attr).ok().flatten() {
            if let AttributeValue::Flag(v) = attr_value.value() {
                return v;
            }
        }
        false
    }

    fn get_member_offset(
        &self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<u64> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_data_member_location).ok()? {
            match attr_value.value() {
                AttributeValue::Udata(v) => return Some(v),
                AttributeValue::Data1(v) => return Some(v as u64),
                AttributeValue::Data2(v) => return Some(v as u64),
                AttributeValue::Data4(v) => return Some(v as u64),
                AttributeValue::Data8(v) => return Some(v),
                AttributeValue::Exprloc(expr) => {
                    // Try to evaluate simple DW_OP_plus_uconst expressions
                    let mut reader = expr.0;
                    let encoding = unit.header.encoding();
                    if let Ok(gimli::Operation::PlusConstant { value }) =
                        gimli::Operation::parse(&mut reader, encoding)
                    {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn get_ref_attr(
        &self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<usize> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::UnitRef(offset) => return Some(offset.0),
                AttributeValue::DebugInfoRef(offset) => {
                    // This is an absolute reference (DW_FORM_ref_addr)
                    // Convert to unit-relative offset if within this unit
                    let unit_base = unit
                        .header
                        .offset()
                        .as_debug_info_offset()
                        .map(|o| o.0)
                        .unwrap_or(0);
                    if offset.0 >= unit_base {
                        return Some(offset.0 - unit_base);
                    }
                    // If it's before our unit, it's a cross-unit reference
                    // which we can't resolve with unit-relative offsets
                    // Return None and let the caller handle it
                }
                _ => {}
            }
        }
        None
    }

    fn get_accessibility(&self, entry: &DebuggingInformationEntry<DwarfReader>) -> Option<String> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_accessibility).ok()? {
            if let AttributeValue::Accessibility(access) = attr_value.value() {
                match access {
                    gimli::DwAccess(1) => return Some("public".to_string()),
                    gimli::DwAccess(2) => return Some("protected".to_string()),
                    gimli::DwAccess(3) => return Some("private".to_string()),
                    _ => {}
                }
            }
        }
        None
    }

    fn get_const_value(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<ConstValue> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_const_value).ok()? {
            match attr_value.value() {
                AttributeValue::Sdata(v) => return Some(ConstValue::Signed(v)),
                AttributeValue::Udata(v) => return Some(ConstValue::Unsigned(v)),
                AttributeValue::Data1(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data2(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data4(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data8(v) => return Some(ConstValue::Unsigned(v)),
                _ => {}
            }
        }
        None
    }
}
