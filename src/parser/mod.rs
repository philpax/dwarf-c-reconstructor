//! DWARF parser implementation
//!
//! This module provides functionality to parse DWARF debugging information
//! from ELF files and extract C/C++ type definitions and function signatures.

mod attributes;
mod method_matcher;
mod type_resolver;

use crate::error::Result;
use crate::types::*;
use gimli::{AttributeValue, DebuggingInformationEntry, Dwarf, Reader};
use object::{Object, ObjectSection, ObjectSymbol};
use std::borrow::Cow;
use std::collections::HashMap;

use attributes::AttributeExtractor;
use type_resolver::TypeResolver;

// Re-export the cross-CU matching function for use in main.rs
pub use method_matcher::cross_cu_match_method_definitions;

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
                .and_then(|section| section.uncompressed_data().ok())
                .unwrap_or(Cow::Borrowed(&[]));

            let relocated_data = apply_relocations(&object, &section_data, id.name());
            if let Cow::Owned(data) = relocated_data {
                section_data_map.insert(id.name(), data.into_boxed_slice());
            } else if let Cow::Owned(data) = section_data {
                // Section was decompressed but not relocated - still need to store it
                section_data_map.insert(id.name(), data.into_boxed_slice());
            }
        }

        // Now load sections with proper references
        // Box ensures the data address is stable even when moved
        let load_section =
            |id: gimli::SectionId| -> std::result::Result<DwarfReader<'a>, gimli::Error> {
                // First check if we have pre-stored data (decompressed and/or relocated)
                let data_ref: &'a [u8] = if let Some(boxed_data) = section_data_map.get(id.name()) {
                    // SAFETY: We're extending the lifetime of the reference from the Box to 'a.
                    // This is safe because:
                    // 1. section_data_map contains the data in Box (stable address)
                    // 2. The Box data will be moved into the DwarfParser struct
                    // 3. The DwarfParser has lifetime 'a
                    // 4. The references won't outlive the parser
                    unsafe { std::slice::from_raw_parts(boxed_data.as_ptr(), boxed_data.len()) }
                } else {
                    // No pre-stored data, use raw section data (for non-compressed, non-relocated sections)
                    object
                        .section_by_name(id.name())
                        .and_then(|section| section.data().ok())
                        .unwrap_or(&[])
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
                method_matcher::match_method_definitions(&mut cu.elements);
                compile_units.push(cu);
            }
        }

        // Third pass: cross-CU matching for methods that couldn't be matched within their CU
        // This handles the case where class declarations are in headers included by multiple CUs,
        // but method definitions are only in one CU
        cross_cu_match_method_definitions(&mut compile_units);

        Ok(compile_units)
    }

    /// Match method declarations across all CUs using linkage names.
    /// This is a public wrapper for cross-CU matching.
    pub fn cross_cu_match_method_definitions(compile_units: &mut [CompileUnit]) {
        cross_cu_match_method_definitions(compile_units);
    }

    // ========================================================================
    // Metadata collection
    // ========================================================================

    fn collect_metadata(&mut self, unit: &DwarfUnit) -> Result<()> {
        let attrs = AttributeExtractor::new(&self.dwarf);
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
                if let Some(name) = attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
                    let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);
                    if let Some(type_offset) = attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
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
                if let Some(name) = attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    self.abstract_origins.insert(abs_offset, name);
                }
            }
        }

        Ok(())
    }

    // ========================================================================
    // Compile unit parsing
    // ========================================================================

    fn parse_compile_unit(&mut self, unit: &DwarfUnit) -> Result<Option<CompileUnit>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let mut entries = unit.entries();

        if let Some((_, entry)) = entries.next_dfs()? {
            if entry.tag() == gimli::DW_TAG_compile_unit {
                let name = attrs
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "unknown".to_string());
                let producer = attrs.get_string_attr(unit, entry, gimli::DW_AT_producer);

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

    // ========================================================================
    // Child element parsing
    // ========================================================================

    fn parse_children(
        &mut self,
        unit: &DwarfUnit,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
        elements: &mut Vec<Element>,
    ) -> Result<()> {
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
                let tag = entry.tag();
                let offset = entry.offset();

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
            }
        }

        Ok(())
    }

    // ========================================================================
    // Namespace parsing
    // ========================================================================

    fn parse_namespace_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Namespace>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        // Create cursor at offset and parse
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        // Extract data from entry before recursive calls
        let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };
        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_namespace_children(unit, name, line, &mut entries)
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

    // ========================================================================
    // Compound type parsing (struct/class/union/enum)
    // ========================================================================

    fn parse_compound_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
        compound_type: &str,
    ) -> Result<Option<Compound>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let name = attrs.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = attrs.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);

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

    fn parse_enum_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<Compound>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let name = attrs.get_string_attr(unit, entry, gimli::DW_AT_name);
        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let byte_size = attrs.get_u64_attr(entry, gimli::DW_AT_byte_size);
        let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);

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

    fn parse_enum_children(
        &mut self,
        unit: &DwarfUnit,
        metadata: CompoundMetadata,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
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
                if let Some(enum_name) = attrs.get_string_attr(unit, child_entry, gimli::DW_AT_name)
                {
                    let value = attrs.get_i64_attr(child_entry, gimli::DW_AT_const_value);
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

    // ========================================================================
    // Member and variable parsing
    // ========================================================================

    fn parse_member(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Variable>> {
        // Extract all attributes first to avoid borrow conflicts with resolve_type
        let (name, line, accessibility, offset, bit_size, bit_offset, const_value, decl_file) = {
            let attrs = AttributeExtractor::new(&self.dwarf);
            let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
                Some(n) => n,
                None => return Ok(None),
            };
            let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
            let accessibility = attrs.get_accessibility(entry);
            let offset = attrs.get_member_offset(unit, entry);
            let bit_size = attrs.get_u64_attr(entry, gimli::DW_AT_bit_size);
            let bit_offset = attrs
                .get_u64_attr(entry, gimli::DW_AT_bit_offset)
                .or_else(|| attrs.get_u64_attr(entry, gimli::DW_AT_data_bit_offset));
            let const_value = attrs.get_const_value(entry);
            let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);
            (
                name,
                line,
                accessibility,
                offset,
                bit_size,
                bit_offset,
                const_value,
                decl_file,
            )
        };

        let type_info = self.resolve_type(unit, entry)?;

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
        // Extract all attributes first to avoid borrow conflicts with resolve_type
        let (offset, accessibility, is_virtual) = {
            let attrs = AttributeExtractor::new(&self.dwarf);
            let offset = attrs.get_member_offset(unit, entry);
            let accessibility = attrs.get_accessibility(entry);
            let is_virtual = attrs.get_bool_attr(entry, gimli::DW_AT_virtuality);
            (offset, accessibility, is_virtual)
        };

        // Get the type of the base class
        let type_info = self.resolve_type(unit, entry)?;
        let type_name = type_info.base_type;

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
        // Extract all attributes first to avoid borrow conflicts with resolve_type
        let (name, line, is_external, const_value, decl_file) = {
            let attrs = AttributeExtractor::new(&self.dwarf);
            let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
                Some(n) => n,
                None => return Ok(None),
            };
            let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
            let is_external = attrs.get_bool_attr(entry, gimli::DW_AT_external);
            let const_value = attrs.get_const_value(entry);
            let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);
            (name, line, is_external, const_value, decl_file)
        };

        let mut type_info = self.resolve_type(unit, entry)?;

        // Check for extern/static
        if is_external {
            type_info.is_extern = true;
        }

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

    // ========================================================================
    // Function parsing
    // ========================================================================

    fn parse_function_at(
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

        // First, extract all attributes that don't require resolve_type
        // to avoid borrow conflicts
        struct FuncAttrs {
            specification_offset: Option<usize>,
            name: Option<String>,
            accessibility: Option<String>,
            is_virtual: bool,
            linkage_name: Option<String>,
            is_declaration: bool,
            line: Option<u64>,
            low_pc: Option<u64>,
            high_pc: Option<u64>,
            is_inline: bool,
            is_external: bool,
            is_artificial: bool,
            decl_file: Option<u64>,
        }

        struct SpecAttrs {
            name: Option<String>,
            accessibility: Option<String>,
            is_virtual: bool,
            linkage_name: Option<String>,
        }

        // Extract main entry attributes
        let main_attrs = {
            let attrs = AttributeExtractor::new(&self.dwarf);
            FuncAttrs {
                specification_offset: attrs.get_ref_attr(unit, entry, gimli::DW_AT_specification),
                name: attrs.get_string_attr(unit, entry, gimli::DW_AT_name),
                accessibility: attrs.get_accessibility(entry),
                is_virtual: attrs.get_bool_attr(entry, gimli::DW_AT_virtuality),
                linkage_name: attrs
                    .get_string_attr(unit, entry, gimli::DW_AT_linkage_name)
                    .or_else(|| attrs.get_string_attr(unit, entry, gimli::DW_AT_MIPS_linkage_name)),
                is_declaration: attrs.get_bool_attr(entry, gimli::DW_AT_declaration),
                line: attrs.get_u64_attr(entry, gimli::DW_AT_decl_line),
                low_pc: attrs.get_u64_attr(entry, gimli::DW_AT_low_pc),
                high_pc: attrs.get_u64_attr(entry, gimli::DW_AT_high_pc),
                is_inline: attrs.get_u64_attr(entry, gimli::DW_AT_inline).is_some(),
                is_external: attrs.get_bool_attr(entry, gimli::DW_AT_external),
                is_artificial: attrs.get_bool_attr(entry, gimli::DW_AT_artificial),
                decl_file: attrs.get_u64_attr(entry, gimli::DW_AT_decl_file),
            }
        };

        // Extract specification entry attributes if present
        let spec_attrs = if let Some(spec_offset) = main_attrs.specification_offset {
            let spec_unit_offset = gimli::UnitOffset(spec_offset);
            let mut spec_entries = unit.entries_at_offset(spec_unit_offset)?;
            if let Some((_, spec_entry)) = spec_entries.next_dfs()? {
                let attrs = AttributeExtractor::new(&self.dwarf);
                Some(SpecAttrs {
                    name: attrs.get_string_attr(unit, spec_entry, gimli::DW_AT_name),
                    accessibility: attrs.get_accessibility(spec_entry),
                    is_virtual: attrs.get_bool_attr(spec_entry, gimli::DW_AT_virtuality),
                    linkage_name: attrs
                        .get_string_attr(unit, spec_entry, gimli::DW_AT_linkage_name)
                        .or_else(|| {
                            attrs.get_string_attr(unit, spec_entry, gimli::DW_AT_MIPS_linkage_name)
                        }),
                })
            } else {
                return Ok(None);
            }
        } else {
            None
        };

        // Now resolve types (needs mutable borrow of self)
        let (
            name,
            return_type,
            accessibility,
            is_virtual_from_spec,
            linkage_name_from_spec,
            spec_abs_offset,
        ) = if let Some(spec_offset) = main_attrs.specification_offset {
            // Follow the specification to get name, return_type, etc from the declaration
            let spec_unit_offset = gimli::UnitOffset(spec_offset);
            let mut spec_entries = unit.entries_at_offset(spec_unit_offset)?;

            if let Some((_, spec_entry)) = spec_entries.next_dfs()? {
                let spec = spec_attrs.unwrap();
                let name = match spec.name {
                    Some(n) => n,
                    None => return Ok(None),
                };
                let return_type = self.resolve_type(unit, spec_entry)?;
                let abs_offset = unit_base + spec_offset;
                (
                    name,
                    return_type,
                    spec.accessibility,
                    spec.is_virtual,
                    spec.linkage_name,
                    Some(abs_offset),
                )
            } else {
                return Ok(None);
            }
        } else {
            // No specification - get name directly from entry
            let name = match main_attrs.name.clone() {
                Some(n) => n,
                None => return Ok(None),
            };

            if main_attrs.is_declaration && !is_method {
                return Ok(None);
            }

            let return_type = self.resolve_type(unit, entry)?;
            (
                name,
                return_type,
                main_attrs.accessibility.clone(),
                main_attrs.is_virtual,
                main_attrs.linkage_name.clone(),
                None,
            )
        };

        // If we have a specification, this is a definition (has body)
        let has_body = main_attrs.specification_offset.is_some() || !main_attrs.is_declaration;

        let mut high_pc = main_attrs.high_pc;

        // high_pc can be either an absolute address or an offset from low_pc
        // If we have both and high_pc is small (likely an offset), convert it
        if let (Some(low), Some(high)) = (main_attrs.low_pc, high_pc) {
            if high < low {
                // high_pc is an offset, convert to absolute address
                high_pc = Some(low + high);
            }
        }

        // Use virtual flag from specification if we have one, otherwise from entry
        let is_virtual = is_virtual_from_spec || main_attrs.is_virtual;
        // Prefer linkage name from entry, fall back to one from specification
        let linkage_name = main_attrs.linkage_name.or(linkage_name_from_spec);

        // Destructor detection by name (~ prefix)
        let is_destructor = name.starts_with('~');
        // Constructor detection will happen during generation when we have class name
        let is_constructor = false;

        // If we have a specification, this is a method definition
        let effective_is_method = is_method || spec_abs_offset.is_some();

        // For method declarations (inside classes), store their absolute offset so definitions can reference them
        let decl_offset = if is_method
            && main_attrs.is_declaration
            && main_attrs.specification_offset.is_none()
        {
            Some(unit_base + offset.0)
        } else {
            None
        };

        let metadata = FunctionMetadata {
            name,
            decl_file: main_attrs.decl_file,
            line: main_attrs.line,
            return_type,
            accessibility,
            has_body,
            is_method: effective_is_method,
            low_pc: main_attrs.low_pc,
            high_pc,
            is_inline: main_attrs.is_inline,
            is_external: main_attrs.is_external,
            is_virtual,
            is_constructor,
            is_destructor,
            linkage_name,
            is_artificial: main_attrs.is_artificial,
            specification_offset: spec_abs_offset,
            decl_offset,
        };

        self.parse_function_children(unit, metadata, &mut entries)
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
            namespace_path: Vec::new(),
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
        let attrs = AttributeExtractor::new(&self.dwarf);
        let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let type_info = self.resolve_type(unit, entry)?;

        Ok(Some(Parameter {
            name,
            type_info,
            line,
        }))
    }

    // ========================================================================
    // Lexical block parsing
    // ========================================================================

    fn parse_lexical_block_at(
        &mut self,
        unit: &DwarfUnit,
        offset: gimli::UnitOffset,
    ) -> Result<Option<LexicalBlock>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let mut entries = unit.entries_at_offset(offset)?;
        let (_, entry) = entries.next_dfs()?.unwrap();

        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);

        self.parse_lexical_block_children(unit, line, &mut entries)
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

    // ========================================================================
    // Inlined subroutine and label parsing
    // ========================================================================

    fn parse_inlined_subroutine(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<InlinedSubroutine>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        // Try to get name from abstract origin
        let name = if let Some(origin_offset) =
            attrs.get_ref_attr(unit, entry, gimli::DW_AT_abstract_origin)
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

        let line = attrs
            .get_u64_attr(entry, gimli::DW_AT_call_line)
            .or_else(|| attrs.get_u64_attr(entry, gimli::DW_AT_decl_line));

        Ok(Some(InlinedSubroutine { name, line }))
    }

    fn parse_label(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<Label>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);

        Ok(Some(Label { name, line }))
    }

    // ========================================================================
    // Typedef alias parsing
    // ========================================================================

    /// Parse a typedef entry and return a TypedefAlias if it points to another typedef or base type
    /// For struct/class/union/enum, only create TypedefAlias if the typedef is in a different file
    /// (same-file typedefs are handled via merging with the compound definition)
    fn parse_typedef_alias(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<Option<TypedefAlias>> {
        let attrs = AttributeExtractor::new(&self.dwarf);
        let name = match attrs.get_string_attr(unit, entry, gimli::DW_AT_name) {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = attrs.get_u64_attr(entry, gimli::DW_AT_decl_line);
        let decl_file = attrs.get_u64_attr(entry, gimli::DW_AT_decl_file);

        // Get what this typedef points to
        let type_offset = match attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
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
                                attrs.get_ref_attr(unit, current_entry, gimli::DW_AT_type)
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
                                attrs.get_bool_attr(current_entry, gimli::DW_AT_declaration);

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
                                attrs.get_u64_attr(current_entry, gimli::DW_AT_decl_file);

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
            let target_type = {
                let mut resolver = TypeResolver::new(
                    AttributeExtractor::new(&self.dwarf),
                    &mut self.type_cache,
                    &self.typedef_map,
                );
                resolver.resolve_type_entry_raw(unit, type_entry)?
            };

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

    // ========================================================================
    // Type resolution (delegated to TypeResolver)
    // ========================================================================

    fn resolve_type(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        let mut resolver = TypeResolver::new(
            AttributeExtractor::new(&self.dwarf),
            &mut self.type_cache,
            &self.typedef_map,
        );
        resolver.resolve_type(unit, entry)
    }
}
