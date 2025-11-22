use gimli::{AttributeValue, DebuggingInformationEntry, Dwarf, Reader};
use object::{Object, ObjectSection};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type DwarfReader = gimli::EndianSlice<'static, gimli::LittleEndian>;
type DwarfUnit = gimli::Unit<DwarfReader>;

#[derive(Debug, Clone)]
struct TypeInfo {
    base_type: String,
    pointer_count: usize,
    array_sizes: Vec<usize>,
    is_const: bool,
    is_static: bool,
    is_extern: bool,
    // For function pointers
    is_function_pointer: bool,
    function_return_type: Option<Box<TypeInfo>>,
    function_params: Vec<TypeInfo>,
}

impl TypeInfo {
    fn new(base_type: String) -> Self {
        TypeInfo {
            base_type,
            pointer_count: 0,
            array_sizes: Vec::new(),
            is_const: false,
            is_static: false,
            is_extern: false,
            is_function_pointer: false,
            function_return_type: None,
            function_params: Vec::new(),
        }
    }

    fn to_string(&self, var_name: &str) -> String {
        let mut result = String::new();

        if self.is_extern {
            result.push_str("extern ");
        }
        if self.is_static {
            result.push_str("static ");
        }

        if self.is_function_pointer {
            // Function pointer: return_type (*var_name)(params)
            if let Some(ret_type) = &self.function_return_type {
                result.push_str(&ret_type.base_type);
                result.push(' ');
                result.push('(');
                // Function pointers always need at least one asterisk
                result.push('*');
                result.push_str(&"*".repeat(self.pointer_count));
                result.push_str(var_name);
                result.push_str(")(");

                for (i, param) in self.function_params.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&param.base_type);
                    if param.pointer_count > 0 {
                        result.push(' ');
                        result.push_str(&"*".repeat(param.pointer_count));
                    }
                }

                result.push(')');
            } else {
                result.push_str("void (*");
                result.push_str(var_name);
                result.push_str(")()");
            }
        } else {
            result.push_str(&self.base_type);
            result.push(' ');
            result.push_str(&"*".repeat(self.pointer_count));
            result.push_str(var_name);

            for size in &self.array_sizes {
                result.push_str(&format!("[{}]", size));
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    type_info: TypeInfo,
    line: Option<u64>,
    accessibility: Option<String>,
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    type_info: TypeInfo,
    line: Option<u64>,
}

#[derive(Debug)]
struct Label {
    name: String,
    line: Option<u64>,
}

#[derive(Debug)]
struct LexicalBlock {
    variables: Vec<Variable>,
    nested_blocks: Vec<LexicalBlock>,
    inlined_calls: Vec<InlinedSubroutine>,
    labels: Vec<Label>,
    line: Option<u64>,
}

#[derive(Debug)]
struct InlinedSubroutine {
    name: String,
    line: Option<u64>,
}

#[derive(Debug)]
struct Function {
    name: String,
    return_type: TypeInfo,
    parameters: Vec<Parameter>,
    variables: Vec<Variable>,
    lexical_blocks: Vec<LexicalBlock>,
    inlined_calls: Vec<InlinedSubroutine>,
    labels: Vec<Label>,
    line: Option<u64>,
    is_method: bool,
    class_name: Option<String>,
    accessibility: Option<String>,
    has_body: bool,
}

#[derive(Debug)]
struct Compound {
    name: Option<String>,
    compound_type: String, // "struct", "union", "enum", "class"
    members: Vec<Variable>,
    methods: Vec<Function>,
    enum_values: Vec<(String, Option<i64>)>,
    line: Option<u64>,
    is_typedef: bool,
    typedef_name: Option<String>,
    typedef_line: Option<u64>,
}

#[derive(Debug)]
struct Namespace {
    name: String,
    line: Option<u64>,
    children: Vec<Element>,
}

#[derive(Debug)]
enum Element {
    Compound(Compound),
    Function(Function),
    Variable(Variable),
    Namespace(Namespace),
}

#[derive(Debug)]
struct CompileUnit {
    name: String,
    elements: Vec<Element>,
}

struct DwarfParser {
    dwarf: Dwarf<DwarfReader>,
    type_cache: HashMap<usize, TypeInfo>,
    typedef_map: HashMap<usize, (String, Option<u64>)>,
    abstract_origins: HashMap<usize, String>,
}

impl DwarfParser {
    fn new(file_data: &'static [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let object = object::File::parse(file_data)?;

        let load_section = |id: gimli::SectionId| -> Result<DwarfReader, gimli::Error> {
            let data = object
                .section_by_name(id.name())
                .and_then(|section| section.data().ok())
                .unwrap_or(&[]);
            Ok(gimli::EndianSlice::new(data, gimli::LittleEndian))
        };

        let dwarf = Dwarf::load(load_section)?;

        Ok(DwarfParser {
            dwarf,
            type_cache: HashMap::new(),
            typedef_map: HashMap::new(),
            abstract_origins: HashMap::new(),
        })
    }

    fn parse(&mut self) -> Result<Vec<CompileUnit>, Box<dyn std::error::Error>> {
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
            if let Some(cu) = self.parse_compile_unit(&unit)? {
                compile_units.push(cu);
            }
        }

        Ok(compile_units)
    }

    fn collect_metadata(&mut self, unit: &DwarfUnit) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = unit.entries();

        while let Some((_, entry)) = entries.next_dfs()? {
            let offset = entry.offset().0;

            // Collect typedefs
            if entry.tag() == gimli::DW_TAG_typedef {
                if let Some(name) = self.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    let line = self.get_u64_attr(entry, gimli::DW_AT_decl_line);
                    if let Some(type_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                        self.typedef_map.insert(type_offset, (name, line));
                    }
                }
            }

            // Collect abstract origins (for inlined functions)
            if entry.tag() == gimli::DW_TAG_subprogram {
                if let Some(name) = self.get_string_attr(unit, entry, gimli::DW_AT_name) {
                    self.abstract_origins.insert(offset, name);
                }
            }
        }

        Ok(())
    }

    fn parse_compile_unit(&mut self, unit: &DwarfUnit) -> Result<Option<CompileUnit>, Box<dyn std::error::Error>> {
        let mut entries = unit.entries();

        if let Some((_, entry)) = entries.next_dfs()? {
            if entry.tag() == gimli::DW_TAG_compile_unit {
                let name = self.get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "unknown".to_string());

                let mut elements = Vec::new();
                self.parse_children(unit, &mut entries, &mut elements)?;

                return Ok(Some(CompileUnit { name, elements }));
            }
        }

        Ok(None)
    }

    fn parse_children(
        &mut self,
        unit: &DwarfUnit,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
        elements: &mut Vec<Element>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut total_depth1 = 0;
        let mut captured = 0;
        let mut absolute_depth = 0;  // Track absolute depth

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
                    gimli::DW_TAG_base_type | gimli::DW_TAG_typedef | gimli::DW_TAG_pointer_type |
                    gimli::DW_TAG_const_type | gimli::DW_TAG_array_type | gimli::DW_TAG_subroutine_type => {
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
        let mut absolute_depth = 1;  // We start at the namespace level (depth 1 from compile unit)

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

        Ok(Some(Namespace { name, line, children }))
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

        let offset_val = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) = if let Some((tname, tline)) = self.typedef_map.get(&offset_val) {
            (true, Some(tname.clone()), *tline)
        } else {
            (false, None, None)
        };

        self.parse_compound_children(unit, name, line, is_typedef, typedef_name, typedef_line, compound_type, &mut entries)
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

        let offset_val = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) = if let Some((tname, tline)) = self.typedef_map.get(&offset_val) {
            (true, Some(tname.clone()), *tline)
        } else {
            (false, None, None)
        };

        self.parse_enum_children(unit, name, line, is_typedef, typedef_name, typedef_line, &mut entries)
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

        self.parse_function_children(unit, name, line, return_type, accessibility, has_body, is_method, &mut entries)
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

        // Check if typedef
        let offset = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) = if let Some((tname, tline)) = self.typedef_map.get(&offset) {
            (true, Some(tname.clone()), *tline)
        } else {
            (false, None, None)
        };

        self.parse_compound_children(unit, name, line, is_typedef, typedef_name, typedef_line, compound_type, entries)
    }

    fn parse_compound_children(
        &mut self,
        unit: &DwarfUnit,
        name: Option<String>,
        line: Option<u64>,
        is_typedef: bool,
        typedef_name: Option<String>,
        typedef_line: Option<u64>,
        compound_type: &str,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        let mut members = Vec::new();
        let mut methods = Vec::new();
        let mut absolute_depth = 1;  // We start at the struct/union level (depth 1 from compile unit)

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
                            members.push(var);
                        }
                    }
                    gimli::DW_TAG_subprogram => {
                        // This is a method declaration
                        if let Some(func) = self.parse_function_at(unit, offset, true)? {
                            methods.push(func);
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

        // Check if typedef
        let offset = entry.offset().0;
        let (is_typedef, typedef_name, typedef_line) = if let Some((tname, tline)) = self.typedef_map.get(&offset) {
            (true, Some(tname.clone()), *tline)
        } else {
            (false, None, None)
        };

        self.parse_enum_children(unit, name, line, is_typedef, typedef_name, typedef_line, entries)
    }

    fn parse_enum_children(
        &mut self,
        unit: &DwarfUnit,
        name: Option<String>,
        line: Option<u64>,
        is_typedef: bool,
        typedef_name: Option<String>,
        typedef_line: Option<u64>,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Compound>, Box<dyn std::error::Error>> {
        let mut enum_values = Vec::new();
        let mut absolute_depth = 1;  // We start at the enum level (depth 1 from compile unit)

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
                if let Some(enum_name) = self.get_string_attr(unit, child_entry, gimli::DW_AT_name) {
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

        Ok(Some(Variable {
            name,
            type_info,
            line,
            accessibility,
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

        Ok(Some(Variable {
            name,
            type_info,
            line,
            accessibility: None,
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

        self.parse_function_children(unit, name, line, return_type, accessibility, has_body, is_method, entries)
    }

    fn parse_function_children(
        &mut self,
        unit: &DwarfUnit,
        name: String,
        line: Option<u64>,
        return_type: TypeInfo,
        accessibility: Option<String>,
        has_body: bool,
        is_method: bool,
        entries: &mut gimli::EntriesCursor<DwarfReader>,
    ) -> Result<Option<Function>, Box<dyn std::error::Error>> {
        let mut parameters = Vec::new();
        let mut variables = Vec::new();
        let mut lexical_blocks = Vec::new();
        let mut inlined_calls = Vec::new();
        let mut labels = Vec::new();
        let mut absolute_depth = 1;  // We start at the function level (depth 1 from compile unit)

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
        let mut absolute_depth = 2;  // We start at the lexical block level (depth 2 from compile unit)

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
        let name = if let Some(origin_offset) = self.get_ref_attr(unit, entry, gimli::DW_AT_abstract_origin) {
            self.abstract_origins.get(&origin_offset).cloned()
        } else {
            None
        };

        let name = match name {
            Some(n) => n,
            None => return Ok(None),
        };

        let line = self.get_u64_attr(entry, gimli::DW_AT_call_line)
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
        // Check cache
        if let Some(cached) = self.type_cache.get(&offset) {
            return Ok(cached.clone());
        }

        let unit_offset = gimli::UnitOffset(offset);
        let mut entries = unit.entries_at_offset(unit_offset)?;

        if let Some((_, type_entry)) = entries.next_dfs()? {
            let type_info = self.resolve_type_entry(unit, type_entry)?;
            self.type_cache.insert(offset, type_info.clone());
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
                let name = self.get_string_attr(unit, entry, gimli::DW_AT_name)
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
                let base_offset = self.get_ref_attr(unit, entry, gimli::DW_AT_type)
                    .unwrap_or(0);
                let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;

                // Get array dimensions from subrange children
                let mut entries = unit.entries_at_offset(entry.offset())?;
                entries.next_dfs()?; // Skip the array type itself

                while let Some((depth, child_entry)) = entries.next_dfs()? {
                    if depth == 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_subrange_type {
                        let size = if let Some(count) = self.get_u64_attr(child_entry, gimli::DW_AT_count) {
                            count as usize
                        } else if let Some(upper) = self.get_u64_attr(child_entry, gimli::DW_AT_upper_bound) {
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
            gimli::DW_TAG_typedef => {
                let name = self.get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "void".to_string());
                Ok(TypeInfo::new(name))
            }
            gimli::DW_TAG_structure_type | gimli::DW_TAG_class_type |
            gimli::DW_TAG_union_type | gimli::DW_TAG_enumeration_type => {
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

                while let Some((depth, child_entry)) = entries.next_dfs()? {
                    if depth == 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_formal_parameter {
                        if let Some(param_offset) = self.get_ref_attr(unit, child_entry, gimli::DW_AT_type) {
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
        unit: &DwarfUnit,
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
            match attr_value.value() {
                AttributeValue::Flag(v) => return v,
                _ => {}
            }
        }
        false
    }

    fn get_ref_attr(
        &self,
        _unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<usize> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::UnitRef(offset) => return Some(offset.0),
                _ => {}
            }
        }
        None
    }

    fn get_accessibility(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<String> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_accessibility).ok()? {
            match attr_value.value() {
                AttributeValue::Udata(1) => return Some("public".to_string()),
                AttributeValue::Udata(2) => return Some("protected".to_string()),
                AttributeValue::Udata(3) => return Some("private".to_string()),
                _ => {}
            }
        }
        None
    }
}

// Code generation
struct CodeGenerator {
    output: String,
    indent_level: usize,
}

impl CodeGenerator {
    fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn write_line(&mut self, line: &str) {
        self.output.push_str(&self.indent());
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn write_line_comment(&mut self, line: &str, comment: &str) {
        self.output.push_str(&self.indent());
        self.output.push_str(line);
        self.output.push_str(" //");
        self.output.push_str(comment);
        self.output.push('\n');
    }

    fn generate_compile_unit(&mut self, cu: &CompileUnit) {
        self.write_line_comment("", &cu.name);
        self.output.push('\n');

        for element in &cu.elements {
            self.generate_element(element);
            self.output.push('\n');
        }

        self.write_line_comment("", &cu.name);
    }

    fn generate_element(&mut self, element: &Element) {
        match element {
            Element::Compound(c) => self.generate_compound(c),
            Element::Function(f) => self.generate_function(f),
            Element::Variable(v) => self.generate_global_variable(v),
            Element::Namespace(ns) => self.generate_namespace(ns),
        }
    }

    fn generate_namespace(&mut self, ns: &Namespace) {
        let line_comment = ns.line.map(|l| format!("//{}", l)).unwrap_or_default();
        self.write_line(&format!("namespace {} {{ {}", ns.name, line_comment));
        self.indent_level += 1;

        for (i, child) in ns.children.iter().enumerate() {
            if i > 0 {
                self.output.push('\n');
            }
            self.generate_element(child);
        }

        self.indent_level -= 1;
        self.write_line(&format!("}} //{}", ns.name));
    }

    fn generate_compound(&mut self, compound: &Compound) {
        // Check if we should merge typedef
        let use_typedef = compound.is_typedef && compound.typedef_name.is_some();

        if compound.compound_type == "enum" {
            self.generate_enum(compound, use_typedef);
        } else if compound.compound_type == "class" {
            self.generate_class(compound);
        } else {
            self.generate_struct_or_union(compound, use_typedef);
        }
    }

    fn generate_enum(&mut self, compound: &Compound, use_typedef: bool) {
        let mut opening = String::new();

        if use_typedef {
            opening.push_str("typedef ");
        }

        opening.push_str(&compound.compound_type);
        opening.push(' ');

        if let Some(ref name) = compound.name {
            opening.push_str(name);
            opening.push(' ');
        }

        opening.push('{');

        if let Some(line) = compound.line {
            opening.push_str(&format!(" //{}", line));
        }

        self.write_line(&opening);

        // Enum values
        self.indent_level += 1;
        for (name, value) in &compound.enum_values {
            if let Some(v) = value {
                self.write_line(&format!("{} = {},", name, v));
            } else {
                self.write_line(&format!("{},", name));
            }
        }
        self.indent_level -= 1;

        let mut closing = String::from("}");
        if use_typedef {
            if let Some(ref tname) = compound.typedef_name {
                closing.push(' ');
                closing.push_str(tname);
            }
        }
        closing.push(';');

        if let Some(tline) = compound.typedef_line {
            closing.push_str(&format!(" //{}", tline));
        }

        self.write_line(&closing);
    }

    fn generate_struct_or_union(&mut self, compound: &Compound, use_typedef: bool) {
        if compound.members.is_empty() {
            // Empty struct/union - just output typedef or declaration
            let mut line = String::new();

            if use_typedef {
                line.push_str("typedef ");
            }

            line.push_str(&compound.compound_type);
            line.push(' ');

            if let Some(ref name) = compound.name {
                line.push_str(name);
                line.push(' ');
            }

            if use_typedef {
                if let Some(ref tname) = compound.typedef_name {
                    line.push_str(tname);
                }
            }

            line.push(';');

            if let Some(line_num) = compound.typedef_line.or(compound.line) {
                line.push_str(&format!(" //{}", line_num));
            }

            self.write_line(&line);
        } else {
            // Struct/union with members
            let mut opening = String::new();

            if use_typedef {
                opening.push_str("typedef ");
            }

            opening.push_str(&compound.compound_type);
            opening.push(' ');

            if let Some(ref name) = compound.name {
                opening.push_str(name);
                opening.push(' ');
            }

            opening.push('{');

            if let Some(line) = compound.line {
                opening.push_str(&format!(" //{}", line));
            }

            self.write_line(&opening);

            // Members grouped by line
            self.indent_level += 1;
            let member_refs: Vec<_> = compound.members.iter().collect();
            self.generate_members(&member_refs);
            self.indent_level -= 1;

            let mut closing = String::from("}");
            if use_typedef {
                if let Some(ref tname) = compound.typedef_name {
                    closing.push(' ');
                    closing.push_str(tname);
                }
            }
            closing.push(';');

            if let Some(tline) = compound.typedef_line {
                closing.push_str(&format!(" //{}", tline));
            }

            self.write_line(&closing);
        }
    }

    fn generate_class(&mut self, compound: &Compound) {
        let mut opening = format!("class {}", compound.name.as_ref().unwrap_or(&String::from("unnamed")));
        opening.push_str(" {");

        if let Some(line) = compound.line {
            opening.push_str(&format!(" //{}", line));
        }

        self.write_line(&opening);

        // Group members and methods by accessibility
        let mut public_members: Vec<&Variable> = Vec::new();
        let mut protected_members: Vec<&Variable> = Vec::new();
        let mut private_members: Vec<&Variable> = Vec::new();

        for member in &compound.members {
            match member.accessibility.as_deref() {
                Some("public") => public_members.push(member),
                Some("protected") => protected_members.push(member),
                _ => private_members.push(member),
            }
        }

        let mut public_methods: Vec<&Function> = Vec::new();
        let mut protected_methods: Vec<&Function> = Vec::new();
        let mut private_methods: Vec<&Function> = Vec::new();

        for method in &compound.methods {
            match method.accessibility.as_deref() {
                Some("public") => public_methods.push(method),
                Some("protected") => protected_methods.push(method),
                _ => private_methods.push(method),
            }
        }

        // Write sections
        if !private_members.is_empty() || !private_methods.is_empty() {
            self.indent_level += 1;
            self.write_line("private:");
            self.generate_members(private_members.iter().copied().collect::<Vec<_>>().as_slice());
            for method in &private_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !protected_members.is_empty() || !protected_methods.is_empty() {
            self.indent_level += 1;
            self.write_line("protected:");
            self.generate_members(protected_members.iter().copied().collect::<Vec<_>>().as_slice());
            for method in &protected_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !public_members.is_empty() || !public_methods.is_empty() {
            self.indent_level += 1;
            self.write_line("public:");
            self.generate_members(public_members.iter().copied().collect::<Vec<_>>().as_slice());
            for method in &public_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        self.write_line("};");
    }

    fn generate_members(&mut self, members: &[&Variable]) {
        // Group by line number and type compatibility
        let mut lines: HashMap<u64, Vec<&Variable>> = HashMap::new();
        let mut no_line_vars = Vec::new();

        for member in members {
            if let Some(line) = member.line {
                lines.entry(line).or_insert_with(Vec::new).push(member);
            } else {
                no_line_vars.push(member);
            }
        }

        // Generate grouped members
        let mut sorted_lines: Vec<_> = lines.iter().collect();
        sorted_lines.sort_by_key(|(line, _)| *line);

        for (line, vars) in sorted_lines {
            // Group by type compatibility
            let mut type_groups: Vec<Vec<&Variable>> = Vec::new();

            for var in vars {
                let mut added = false;
                for group in &mut type_groups {
                    if self.types_compatible(&group[0].type_info, &var.type_info) {
                        group.push(var);
                        added = true;
                        break;
                    }
                }
                if !added {
                    type_groups.push(vec![var]);
                }
            }

            // Generate declarations
            let mut decls = Vec::new();
            for group in type_groups {
                // Check if this group contains function pointers - they can't be grouped
                if group[0].type_info.is_function_pointer {
                    // Output function pointers individually
                    for var in group {
                        let decl = var.type_info.to_string(&var.name);
                        decls.push(decl);
                    }
                } else {
                    let base_type = &group[0].type_info;
                    let mut var_names = Vec::new();

                    for var in group {
                        let ptr_str = "*".repeat(var.type_info.pointer_count);
                        let mut name_with_array = format!("{}{}", ptr_str, var.name);

                        for size in &var.type_info.array_sizes {
                            name_with_array.push_str(&format!("[{}]", size));
                        }

                        var_names.push(name_with_array);
                    }

                    let mut decl = base_type.base_type.clone();
                    decl.push(' ');
                    decl.push_str(&var_names.join(", "));
                    decls.push(decl);
                }
            }

            let full_decl = decls.join("; ");
            self.write_line_comment(&format!("{};", full_decl), &line.to_string());
        }

        // Variables without line numbers
        for var in no_line_vars {
            self.write_line(&format!("{};", var.type_info.to_string(&var.name)));
        }
    }

    fn types_compatible(&self, t1: &TypeInfo, t2: &TypeInfo) -> bool {
        // Two types are compatible for joining if they have the same base type
        // and differ only in pointer count or array sizes
        t1.base_type == t2.base_type && !t1.is_function_pointer && !t2.is_function_pointer
    }

    fn generate_method(&mut self, func: &Function) {
        if !func.has_body || (func.variables.is_empty() && func.lexical_blocks.is_empty() &&
                               func.inlined_calls.is_empty() && func.labels.is_empty()) {
            // Method declaration only - need to build declaration without embedded line comment
            // so we can put semicolon before the comment
            let decl = self.generate_method_declaration(func);
            self.write_line(&decl);
        } else {
            let decl = self.generate_function_declaration(func);
            self.write_line(&decl);
            self.write_line("{");
            self.indent_level += 1;
            self.generate_function_body(func);
            self.indent_level -= 1;
            self.write_line("}");
        }
    }

    fn generate_method_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Return type
        decl.push_str(&func.return_type.base_type);
        if func.return_type.pointer_count > 0 {
            decl.push_str(&"*".repeat(func.return_type.pointer_count));
        }
        decl.push(' ');

        // Function name
        decl.push_str(&func.name);
        decl.push('(');

        // Parameters (skip 'this' for methods)
        let params: Vec<_> = func.parameters.iter()
            .filter(|p| p.name != "this")
            .collect();

        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                decl.push_str(", ");
            }
            decl.push_str(&param.type_info.to_string(&param.name));
        }
        decl.push_str(");");

        // Add line comment after semicolon
        if let Some(line) = func.line {
            decl.push_str(&format!(" //{}", line));
        }

        decl
    }

    fn generate_function(&mut self, func: &Function) {
        let decl = self.generate_function_declaration(func);

        if !func.has_body || (func.variables.is_empty() && func.lexical_blocks.is_empty() &&
                               func.inlined_calls.is_empty() && func.labels.is_empty()) {
            // Function declaration only - put semicolon on same line
            self.write_line(&format!("{}; {}", decl,
                func.line.map(|l| format!("//{}", l)).unwrap_or_default()));
        } else {
            self.write_line(&decl);

            // Check if we have a single lexical block at top level with no other variables
            let single_block = func.lexical_blocks.len() == 1 &&
                               func.variables.is_empty() &&
                               func.inlined_calls.is_empty() &&
                               func.labels.is_empty();

            if single_block {
                // Use the lexical block's braces
                self.generate_lexical_block(&func.lexical_blocks[0]);
            } else {
                // Add our own braces
                self.write_line("{");
                self.indent_level += 1;
                self.generate_function_body(func);
                self.indent_level -= 1;
                self.write_line("}");
            }
        }
    }

    fn generate_function_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Return type
        decl.push_str(&func.return_type.base_type);
        if func.return_type.pointer_count > 0 {
            decl.push_str(&"*".repeat(func.return_type.pointer_count));
        }
        decl.push(' ');

        // Function name
        decl.push_str(&func.name);
        decl.push('(');

        // Parameters (skip 'this' for methods)
        let params: Vec<_> = func.parameters.iter()
            .filter(|p| !(func.is_method && p.name == "this"))
            .collect();

        if params.is_empty() {
            decl.push(')');
            if let Some(line) = func.line {
                decl.push_str(&format!(" //{}", line));
            }
        } else {
            // Check if all params are on same line as function
            let all_same_line = params.iter().all(|p| p.line == func.line);

            if all_same_line {
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        decl.push_str(", ");
                    }
                    decl.push_str(&param.type_info.to_string(&param.name));
                }
                decl.push(')');
                if let Some(line) = func.line {
                    decl.push_str(&format!(" //{}", line));
                }
            } else {
                // Parameters on different lines
                if let Some(line) = func.line {
                    decl.push_str(&format!(" //{}", line));
                }
                decl.push('\n');

                // Group parameters by line
                let mut param_lines: HashMap<Option<u64>, Vec<&Parameter>> = HashMap::new();
                for param in &params {
                    param_lines.entry(param.line).or_insert_with(Vec::new).push(param);
                }

                let mut sorted_lines: Vec<_> = param_lines.iter().collect();
                sorted_lines.sort_by_key(|(line, _)| *line);

                for (idx, (line, params_at_line)) in sorted_lines.iter().enumerate() {
                    decl.push_str(&self.indent());
                    decl.push_str("        ");

                    for (i, param) in params_at_line.iter().enumerate() {
                        if i > 0 {
                            decl.push_str(", ");
                        }
                        decl.push_str(&param.type_info.to_string(&param.name));
                    }

                    if idx == sorted_lines.len() - 1 {
                        decl.push(')');
                    } else {
                        decl.push(',');
                    }

                    if let Some(l) = line {
                        decl.push_str(&format!(" //{}", l));
                    }

                    if idx < sorted_lines.len() - 1 {
                        decl.push('\n');
                    }
                }
            }
        }

        decl
    }

    fn generate_function_body(&mut self, func: &Function) {
        // Variables
        self.generate_variables(&func.variables);

        // Inlined calls
        for inlined in &func.inlined_calls {
            let line_comment = inlined.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.write_line(&format!("{}();{}", inlined.name, line_comment));
        }

        // Labels
        for label in &func.labels {
            let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.write_line(&format!("{}:{}", label.name, line_comment));
        }

        // Lexical blocks
        for block in &func.lexical_blocks {
            self.generate_lexical_block(block);
        }
    }

    fn generate_variables(&mut self, variables: &[Variable]) {
        // Group by line number and type compatibility
        let mut lines: HashMap<u64, Vec<&Variable>> = HashMap::new();
        let mut no_line_vars = Vec::new();

        for var in variables {
            if let Some(line) = var.line {
                lines.entry(line).or_insert_with(Vec::new).push(var);
            } else {
                no_line_vars.push(var);
            }
        }

        // Generate grouped variables
        let mut sorted_lines: Vec<_> = lines.iter().collect();
        sorted_lines.sort_by_key(|(line, _)| *line);

        for (line, vars) in sorted_lines {
            // Group by type compatibility
            let mut type_groups: Vec<Vec<&Variable>> = Vec::new();

            for var in vars {
                let mut added = false;
                for group in &mut type_groups {
                    if self.types_compatible(&group[0].type_info, &var.type_info) {
                        group.push(var);
                        added = true;
                        break;
                    }
                }
                if !added {
                    type_groups.push(vec![var]);
                }
            }

            // Generate declarations
            let mut decls = Vec::new();
            for group in type_groups {
                // Check if this group contains function pointers - they can't be grouped
                if group[0].type_info.is_function_pointer {
                    // Output function pointers individually
                    for var in group {
                        let decl = var.type_info.to_string(&var.name);
                        decls.push(decl);
                    }
                } else {
                    let base_type = &group[0].type_info;
                    let mut var_names = Vec::new();

                    for var in group {
                        let ptr_str = "*".repeat(var.type_info.pointer_count);
                        let mut name_with_array = format!("{}{}", ptr_str, var.name);

                        for size in &var.type_info.array_sizes {
                            name_with_array.push_str(&format!("[{}]", size));
                        }

                        var_names.push(name_with_array);
                    }

                    let mut decl = base_type.base_type.clone();
                    decl.push(' ');
                    decl.push_str(&var_names.join(", "));
                    decls.push(decl);
                }
            }

            let full_decl = decls.join("; ");
            self.write_line_comment(&format!("{};", full_decl), &line.to_string());
        }

        // Variables without line numbers
        for var in no_line_vars {
            self.write_line(&format!("{};", var.type_info.to_string(&var.name)));
        }
    }

    fn generate_lexical_block(&mut self, block: &LexicalBlock) {
        self.write_line("{");
        self.indent_level += 1;

        // Variables
        self.generate_variables(&block.variables);

        // Inlined calls
        for inlined in &block.inlined_calls {
            let line_comment = inlined.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.write_line(&format!("{}();{}", inlined.name, line_comment));
        }

        // Labels
        for label in &block.labels {
            let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.write_line(&format!("{}:{}", label.name, line_comment));
        }

        // Nested blocks
        for nested in &block.nested_blocks {
            self.generate_lexical_block(nested);
        }

        self.indent_level -= 1;
        self.write_line("}");
    }

    fn generate_global_variable(&mut self, var: &Variable) {
        let line_comment = var.line.map(|l| format!(" //{}", l)).unwrap_or_default();
        self.write_line(&format!("{};{}", var.type_info.to_string(&var.name), line_comment));
    }

    fn get_output(self) -> String {
        self.output
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <elf_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read file into static memory
    let file_data = fs::read(file_path)?;
    let static_data: &'static [u8] = Box::leak(file_data.into_boxed_slice());

    // Parse DWARF
    let mut parser = DwarfParser::new(static_data)?;
    let compile_units = parser.parse()?;

    // Generate code for each compile unit
    let output_dir = Path::new("output");
    fs::create_dir_all(output_dir)?;

    for cu in &compile_units {
        let mut generator = CodeGenerator::new();
        generator.generate_compile_unit(cu);

        // Determine output file name
        let output_name = if cu.name.is_empty() {
            "unknown.c".to_string()
        } else {
            // Extract just the filename from the path
            Path::new(&cu.name)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.c")
                .to_string()
        };

        let output_path = output_dir.join(&output_name);
        fs::write(&output_path, generator.get_output())?;
        println!("Generated: {}", output_path.display());
    }

    Ok(())
}
