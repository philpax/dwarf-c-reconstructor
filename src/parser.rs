//! DWARF parser implementation

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

pub struct DwarfParser {
    dwarf: Dwarf<DwarfReader>,
    type_cache: HashMap<usize, TypeInfo>,
    typedef_map: HashMap<usize, (String, Option<u64>)>,
    abstract_origins: HashMap<usize, String>,
    // Keep the section data alive
    _section_data: Vec<Vec<u8>>,
}
#[allow(dead_code)] // Some parser methods are called via offset-based parsing
#[allow(clippy::too_many_arguments)] // Parser methods need many parameters from DWARF
#[allow(clippy::while_let_loop)] // Some loops are clearer with explicit match
impl DwarfParser {
    pub fn new(file_data: &'static [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let object = object::File::parse(file_data)?;

        let load_section = |id: gimli::SectionId| -> Result<DwarfReader, gimli::Error> {
            let section_data = object
                .section_by_name(id.name())
                .and_then(|section| section.data().ok())
                .unwrap_or(&[]);

            // Apply relocations if this is an object file
            let relocated_data = apply_relocations(&object, section_data, id.name());

            // Convert to 'static lifetime by leaking
            let static_data: &'static [u8] = match relocated_data {
                Cow::Borrowed(data) => data,
                Cow::Owned(data) => Box::leak(data.into_boxed_slice()),
            };

            Ok(gimli::EndianSlice::new(static_data, gimli::LittleEndian))
        };

        let dwarf = Dwarf::load(load_section)?;

        Ok(DwarfParser {
            dwarf,
            type_cache: HashMap::new(),
            typedef_map: HashMap::new(),
            abstract_origins: HashMap::new(),
            _section_data: Vec::new(),
        })
    }

    pub fn parse(&mut self) -> Result<Vec<CompileUnit>, Box<dyn std::error::Error>> {
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
                // Third pass: match method declarations with definitions
                Self::match_method_definitions(&mut cu.elements);
                compile_units.push(cu);
            }
        }

        Ok(compile_units)
    }

    /// Match method declarations in classes with their definitions at top level
    fn match_method_definitions(elements: &mut Vec<Element>) {
        use std::collections::HashMap;

        // Build a map of linkage names to cloned function data
        let mut definitions: HashMap<String, (Vec<Parameter>, Vec<Variable>, Vec<LexicalBlock>, Vec<InlinedSubroutine>, Vec<Label>, bool, Option<u64>, Option<u64>)> = HashMap::new();

        for element in elements.iter() {
            if let Element::Function(func) = element {
                if !func.is_method {
                    if let Some(ref linkage_name) = func.linkage_name {
                        definitions.insert(
                            linkage_name.clone(),
                            (
                                func.parameters.clone(),
                                func.variables.clone(),
                                func.lexical_blocks.clone(),
                                func.inlined_calls.clone(),
                                func.labels.clone(),
                                func.has_body,
                                func.low_pc,
                                func.high_pc,
                            ),
                        );
                    }
                }
            }
        }

        // Match methods with definitions
        for element in elements.iter_mut() {
            match element {
                Element::Compound(compound) => {
                    for method in &mut compound.methods {
                        if let Some(ref linkage_name) = method.linkage_name {
                            if let Some((params, vars, blocks, inlined, labels, has_body, low_pc, high_pc)) = definitions.get(linkage_name) {
                                // Found matching definition, copy parameters and body info
                                method.parameters = params.clone();
                                method.variables = vars.clone();
                                method.lexical_blocks = blocks.clone();
                                method.inlined_calls = inlined.clone();
                                method.labels = labels.clone();
                                method.has_body = *has_body;
                                method.low_pc = *low_pc;
                                method.high_pc = *high_pc;
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

        // Mark top-level functions that are method definitions
        let mut method_linkage_names: HashMap<String, String> = HashMap::new();
        for element in elements.iter() {
            if let Element::Compound(compound) = element {
                if let Some(ref class_name) = compound.name {
                    for method in &compound.methods {
                        if let Some(ref linkage_name) = method.linkage_name {
                            method_linkage_names.insert(linkage_name.clone(), class_name.clone());
                        }
                    }
                }
            }
        }

        for element in elements.iter_mut() {
            if let Element::Function(func) = element {
                if !func.is_method {
                    if let Some(ref linkage_name) = func.linkage_name {
                        if let Some(class_name) = method_linkage_names.get(linkage_name) {
                            // Mark this function as a method and set its class name
                            func.is_method = true;
                            func.class_name = Some(class_name.clone());
                        }
                    }
                }
            }
        }
    }

    fn collect_metadata(&mut self, unit: &DwarfUnit) -> Result<(), Box<dyn std::error::Error>> {
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
                    if let Some(type_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                        // Convert to absolute offset
                        let abs_type_offset = unit_base + type_offset;
                        self.typedef_map.insert(abs_type_offset, (name, line));
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

    fn parse_compile_unit(
        &mut self,
        unit: &DwarfUnit,
    ) -> Result<Option<CompileUnit>, Box<dyn std::error::Error>> {
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
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                    // Skip base_type and typedef at top level - they'll be resolved when referenced
                    gimli::DW_TAG_base_type
                    | gimli::DW_TAG_typedef
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
    ) -> Result<Option<Namespace>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        self.parse_compound_offset(unit, offset, compound_type)
    }

    fn parse_enum_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        self.parse_enum_offset(unit, offset)
    }

    fn parse_function_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        is_method: bool,
    ) -> Result<Option<Function>, Box<dyn std::error::Error>> {
        self.parse_function_offset(unit, offset, is_method)
    }

    fn parse_lexical_block_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<LexicalBlock>, Box<dyn std::error::Error>> {
        self.parse_lexical_block_offset(unit, offset)
    }

    fn parse_namespace_children(
        &mut self,
        unit: &DwarfUnit,
        name: String,
        line: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Namespace>, Box<dyn std::error::Error>> {
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
                    gimli::DW_TAG_subprogram => {
                        if let Some(func) = self.parse_function_at(unit, offset, false)? {
                            children.push(Element::Function(func));
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
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
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

        let (is_typedef, typedef_name, typedef_line) =
            if let Some((tname, tline)) = self.typedef_map.get(&abs_offset) {
                (true, Some(tname.clone()), *tline)
            } else {
                (false, None, None)
            };

        self.parse_compound_children(
            unit,
            name,
            line,
            byte_size,
            is_typedef,
            typedef_name,
            typedef_line,
            compound_type,
            decl_file,
            &mut entries,
        )
    }

    fn parse_enum_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
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

        let (is_typedef, typedef_name, typedef_line) =
            if let Some((tname, tline)) = self.typedef_map.get(&abs_offset) {
                (true, Some(tname.clone()), *tline)
            } else {
                (false, None, None)
            };

        self.parse_enum_children(
            unit,
            name,
            line,
            byte_size,
            is_typedef,
            typedef_name,
            typedef_line,
            decl_file,
            &mut entries,
        )
    }

    fn parse_function_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        is_method: bool,
    ) -> Result<Option<Function>, Box<dyn std::error::Error>> {
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

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

        self.parse_function_children(
            unit,
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
            &mut entries,
        )
    }

    fn parse_lexical_block_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<LexicalBlock>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Check if typedef
        let offset = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) =
            if let Some((tname, tline)) = self.typedef_map.get(&offset) {
                (true, Some(tname.clone()), *tline)
            } else {
                (false, None, None)
            };

        self.parse_compound_children(
            unit,
            name,
            line,
            byte_size,
            is_typedef,
            typedef_name,
            typedef_line,
            compound_type,
            decl_file,
            entries,
        )
    }

    fn parse_compound_children(
        &mut self,
        unit: &DwarfUnit,
        name: Option<String>,
        line: Option<u64>,
        byte_size: Option<u64>,
        is_typedef: bool,
        typedef_name: Option<String>,
        typedef_line: Option<u64>,
        compound_type: &str,
        decl_file: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        let mut members = Vec::new();
        let mut methods = Vec::new();
        let mut base_classes = Vec::new();
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
                            func.class_name = name.clone();
                            methods.push(func);
                        }
                    }
                    gimli::DW_TAG_inheritance => {
                        if let Some(base) = self.parse_inheritance(unit, child_entry)? {
                            base_classes.push(base);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(Compound {
            name,
            compound_type: compound_type.to_string(),
            members,
            methods,
            enum_values: Vec::new(),
            line,
            is_typedef,
            typedef_name,
            typedef_line,
            byte_size,
            base_classes,
            is_virtual,
            decl_file,
        }))
    }

    fn parse_enum(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        let name = self.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = self.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = self.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Check if typedef
        let offset = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) =
            if let Some((tname, tline)) = self.typedef_map.get(&offset) {
                (true, Some(tname.clone()), *tline)
            } else {
                (false, None, None)
            };

        self.parse_enum_children(
            unit,
            name,
            line,
            byte_size,
            is_typedef,
            typedef_name,
            typedef_line,
            decl_file,
            entries,
        )
    }

    fn parse_enum_children(
        &mut self,
        unit: &DwarfUnit,
        name: Option<String>,
        line: Option<u64>,
        byte_size: Option<u64>,
        is_typedef: bool,
        typedef_name: Option<String>,
        typedef_line: Option<u64>,
        decl_file: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
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
            name,
            compound_type: "enum".to_string(),
            members: Vec::new(),
            methods: Vec::new(),
            enum_values,
            line,
            is_typedef,
            typedef_name,
            typedef_line,
            byte_size,
            base_classes: Vec::new(),
            decl_file,
            is_virtual: false,
        }))
    }

    fn parse_member(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Variable>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<BaseClass>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<Variable>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<Function>, Box<dyn std::error::Error>> {
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

        self.parse_function_children(
            unit,
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
            entries,
        )
    }

    fn parse_function_children(
        &mut self,
        unit: &DwarfUnit,
        name: String,
        decl_file: Option<u64>,
        line: Option<u64>,
        return_type: TypeInfo,
        accessibility: Option<String>,
        has_body: bool,
        is_method: bool,
        low_pc: Option<u64>,
        high_pc: Option<u64>,
        is_inline: bool,
        is_external: bool,
        is_virtual: bool,
        is_constructor: bool,
        is_destructor: bool,
        linkage_name: Option<String>,
        is_artificial: bool,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Function>, Box<dyn std::error::Error>> {
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
            name,
            return_type,
            parameters,
            variables,
            lexical_blocks,
            inlined_calls,
            labels,
            line,
            is_method,
            class_name: None,
            accessibility,
            has_body,
            low_pc,
            high_pc,
            is_inline,
            is_external,
            is_virtual,
            is_constructor,
            is_destructor,
            linkage_name,
            is_artificial,
            decl_file,
        }))
    }

    fn parse_parameter(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Parameter>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<LexicalBlock>, Box<dyn std::error::Error>> {
        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_lexical_block_children(unit, line, entries)
    }

    fn parse_lexical_block_children(
        &mut self,
        unit: &DwarfUnit,
        line: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<LexicalBlock>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<InlinedSubroutine>, Box<dyn std::error::Error>> {
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
    ) -> Result<Option<Label>, Box<dyn std::error::Error>> {
        let name = match self.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);

        Ok(Some(Label { name, line }))
    }

    fn resolve_type(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo, Box<dyn std::error::Error>> {
        let type_offset = match self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
            Some(offset) => offset,
            None => return Ok(TypeInfo::new("void".to_string())),
        };

        self.resolve_type_from_offset(unit, type_offset)
    }

    fn resolve_type_from_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: usize,
    ) -> Result<TypeInfo, Box<dyn std::error::Error>> {
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
    ) -> Result<TypeInfo, Box<dyn std::error::Error>> {
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

                    // Check if it has a typedef
                    let offset = entry.offset().0;
                    if let Some((typedef_name, _)) = self.typedef_map.get(&offset) {
                        Ok(TypeInfo::new(typedef_name.clone()))
                    } else {
                        Ok(TypeInfo::new(format!("{}{}", prefix, n)))
                    }
                } else {
                    // Anonymous type, check for typedef
                    let offset = entry.offset().0;
                    if let Some((typedef_name, _)) = self.typedef_map.get(&offset) {
                        Ok(TypeInfo::new(typedef_name.clone()))
                    } else {
                        Ok(TypeInfo::new("void".to_string()))
                    }
                }
            }
            gimli::DW_TAG_subroutine_type => {
                // Function pointer
                let mut func_type = TypeInfo::new("void".to_string());
                func_type.is_function_pointer = true;

                // Get return type
                if let Some(ret_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let ret_type = self.resolve_type_from_offset(unit, ret_offset)?;
                    func_type.function_return_type = Some(Box::new(ret_type));
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
                        if let Some(param_offset) =
                            self.get_ref_attr(unit, child_entry, gimli::DW_AT_type)
                        {
                            let param_type = self.resolve_type_from_offset(unit, param_offset)?;
                            func_type.function_params.push(param_type);
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
        _unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<usize> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            if let AttributeValue::UnitRef(offset) = attr_value.value() {
                return Some(offset.0);
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
