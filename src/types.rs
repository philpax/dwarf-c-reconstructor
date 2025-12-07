//! Type definitions for DWARF C reconstructor

pub type DwarfReader<'a> = gimli::EndianSlice<'a, gimli::LittleEndian>;
pub type DwarfUnit<'a> = gimli::Unit<DwarfReader<'a>>;

/// Information about a typedef collected during metadata pass.
/// Maps a type offset to its typedef name and source location.
#[derive(Debug, Clone)]
pub struct TypedefInfo {
    pub name: String,
    pub line: Option<u64>,
    pub decl_file: Option<u64>,
}

/// Metadata for parsing compound types (struct/class/union).
/// Groups related fields passed to parse_compound_children.
#[derive(Debug, Clone)]
pub struct CompoundMetadata {
    pub name: Option<String>,
    pub line: Option<u64>,
    pub byte_size: Option<u64>,
    pub is_typedef: bool,
    pub typedef_name: Option<String>,
    pub typedef_line: Option<u64>,
    pub compound_type: String,
    pub decl_file: Option<u64>,
}

/// Metadata for parsing function/method definitions.
/// Groups the many fields needed for parse_function_children.
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub decl_file: Option<u64>,
    pub line: Option<u64>,
    pub return_type: TypeInfo,
    pub accessibility: Option<String>,
    pub has_body: bool,
    pub is_method: bool,
    pub low_pc: Option<u64>,
    pub high_pc: Option<u64>,
    pub is_inline: bool,
    pub is_external: bool,
    pub is_virtual: bool,
    pub is_constructor: bool,
    pub is_destructor: bool,
    pub linkage_name: Option<String>,
    pub is_artificial: bool,
    pub specification_offset: Option<usize>,
    pub decl_offset: Option<usize>,
}

/// Data collected from a method definition that can be applied to declarations.
/// Used for matching method declarations with their definitions.
#[derive(Debug, Clone)]
pub struct MethodDefinition {
    pub parameters: Vec<Parameter>,
    pub variables: Vec<Variable>,
    pub lexical_blocks: Vec<LexicalBlock>,
    pub inlined_calls: Vec<InlinedSubroutine>,
    pub labels: Vec<Label>,
    pub has_body: bool,
    pub low_pc: Option<u64>,
    pub high_pc: Option<u64>,
    pub line: Option<u64>,
}

impl MethodDefinition {
    /// Create a MethodDefinition from a Function.
    pub fn from_function(func: &Function) -> Self {
        MethodDefinition {
            parameters: func.parameters.clone(),
            variables: func.variables.clone(),
            lexical_blocks: func.lexical_blocks.clone(),
            inlined_calls: func.inlined_calls.clone(),
            labels: func.labels.clone(),
            has_body: func.has_body,
            low_pc: func.low_pc,
            high_pc: func.high_pc,
            line: func.line,
        }
    }

    /// Apply this definition's data to a method declaration.
    pub fn apply_to_method(&self, method: &mut Function) {
        method.parameters = self.parameters.clone();
        method.variables = self.variables.clone();
        method.lexical_blocks = self.lexical_blocks.clone();
        method.inlined_calls = self.inlined_calls.clone();
        method.labels = self.labels.clone();
        method.has_body = self.has_body;
        method.low_pc = self.low_pc;
        method.high_pc = self.high_pc;
        // Use line from definition if declaration doesn't have one
        if method.line.is_none() {
            method.line = self.line;
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub base_type: String,
    pub pointer_count: usize,
    pub array_sizes: Vec<usize>,
    pub is_const: bool,
    pub is_volatile: bool,
    pub is_restrict: bool,
    pub is_static: bool,
    pub is_extern: bool,
    pub is_reference: bool,        // C++ lvalue reference (T&)
    pub is_rvalue_reference: bool, // C++ rvalue reference (T&&)
    // For function pointers
    pub is_function_pointer: bool,
    pub function_return_type: Option<Box<TypeInfo>>,
    pub function_params: Vec<TypeInfo>,
}

impl TypeInfo {
    pub fn new(base_type: String) -> Self {
        TypeInfo {
            base_type,
            pointer_count: 0,
            array_sizes: Vec::new(),
            is_const: false,
            is_volatile: false,
            is_restrict: false,
            is_static: false,
            is_extern: false,
            is_reference: false,
            is_rvalue_reference: false,
            is_function_pointer: false,
            function_return_type: None,
            function_params: Vec::new(),
        }
    }

    pub fn to_string(&self, var_name: &str) -> String {
        let mut result = String::new();

        if self.is_extern {
            result.push_str("extern ");
        }
        if self.is_static {
            result.push_str("static ");
        }

        if self.is_function_pointer {
            // Function pointer: return_type (*var_name)(params)
            // Get return type, defaulting to void if not specified
            let ret_base_type = self
                .function_return_type
                .as_ref()
                .map(|t| t.base_type.as_str())
                .unwrap_or("void");
            let ret_is_const = self
                .function_return_type
                .as_ref()
                .map(|t| t.is_const)
                .unwrap_or(false);

            // Add const for return type if present
            if ret_is_const {
                result.push_str("const ");
            }
            result.push_str(ret_base_type);
            result.push_str(" (");
            // Function pointers always need at least one asterisk
            result.push('*');
            result.push_str(&"*".repeat(self.pointer_count));
            result.push_str(var_name);
            result.push_str(")(");

            for (i, param) in self.function_params.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                // Add const for parameter if present
                if param.is_const {
                    result.push_str("const ");
                }
                result.push_str(&param.base_type);
                if param.pointer_count > 0 {
                    result.push_str(&"*".repeat(param.pointer_count));
                }
            }

            result.push(')');
        } else {
            // Add type qualifiers before base type
            if self.is_const {
                result.push_str("const ");
            }
            if self.is_volatile {
                result.push_str("volatile ");
            }
            if self.is_restrict {
                result.push_str("restrict ");
            }
            result.push_str(&self.base_type);
            result.push(' ');

            // References and pointers (attached to variable name)
            if self.is_rvalue_reference {
                result.push_str("&&");
            } else if self.is_reference {
                result.push('&');
            } else if self.pointer_count > 0 {
                result.push_str(&"*".repeat(self.pointer_count));
            }

            result.push_str(var_name);

            for size in &self.array_sizes {
                result.push_str(&format!("[{}]", size));
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_info: TypeInfo,
    pub line: Option<u64>,
    pub accessibility: Option<String>,
    pub offset: Option<u64>,
    pub bit_size: Option<u64>,
    pub bit_offset: Option<u64>,
    pub const_value: Option<ConstValue>,
    pub decl_file: Option<u64>, // File index from DWARF file table
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Signed(i64),
    Unsigned(u64),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_info: TypeInfo,
    pub line: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub line: Option<u64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // line field may be used in future
pub struct LexicalBlock {
    pub variables: Vec<Variable>,
    pub nested_blocks: Vec<LexicalBlock>,
    pub inlined_calls: Vec<InlinedSubroutine>,
    pub labels: Vec<Label>,
    pub line: Option<u64>,
}

impl LexicalBlock {
    /// Calculate the minimum line number from all contents of this block (recursively)
    pub fn min_content_line(&self) -> Option<u64> {
        let mut min_line: Option<u64> = self.line;

        // Check variables
        for var in &self.variables {
            if let Some(line) = var.line {
                min_line = Some(min_line.map_or(line, |m| m.min(line)));
            }
        }

        // Check inlined calls
        for inlined in &self.inlined_calls {
            if let Some(line) = inlined.line {
                min_line = Some(min_line.map_or(line, |m| m.min(line)));
            }
        }

        // Check labels
        for label in &self.labels {
            if let Some(line) = label.line {
                min_line = Some(min_line.map_or(line, |m| m.min(line)));
            }
        }

        // Check nested blocks recursively
        for nested in &self.nested_blocks {
            if let Some(line) = nested.min_content_line() {
                min_line = Some(min_line.map_or(line, |m| m.min(line)));
            }
        }

        min_line
    }
}

#[derive(Debug, Clone)]
pub struct InlinedSubroutine {
    pub name: String,
    pub line: Option<u64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // is_constructor field is used during generation, not read from struct
pub struct Function {
    pub name: String,
    pub return_type: TypeInfo,
    pub parameters: Vec<Parameter>,
    pub variables: Vec<Variable>,
    pub lexical_blocks: Vec<LexicalBlock>,
    pub inlined_calls: Vec<InlinedSubroutine>,
    pub labels: Vec<Label>,
    pub line: Option<u64>,
    pub is_method: bool,
    pub class_name: Option<String>,
    pub accessibility: Option<String>,
    pub has_body: bool,
    pub low_pc: Option<u64>,
    pub high_pc: Option<u64>,
    pub is_inline: bool,
    pub is_external: bool,
    pub is_virtual: bool,
    pub is_constructor: bool,
    pub is_destructor: bool,
    pub linkage_name: Option<String>,
    pub is_artificial: bool,
    pub decl_file: Option<u64>, // File index from DWARF file table
    pub specification_offset: Option<usize>, // Absolute offset of the declaration this definition refers to
    pub decl_offset: Option<usize>,          // Absolute offset of this declaration (for matching)
}

#[derive(Debug, Clone)]
pub struct BaseClass {
    pub type_name: String,
    pub offset: Option<u64>,
    pub accessibility: Option<String>,
    pub is_virtual: bool,
}

#[derive(Debug, Clone)]
pub struct Compound {
    pub name: Option<String>,
    pub compound_type: String, // "struct", "union", "enum", "class"
    pub members: Vec<Variable>,
    pub methods: Vec<Function>,
    pub enum_values: Vec<(String, Option<i64>)>,
    pub line: Option<u64>,
    pub is_typedef: bool,
    pub typedef_name: Option<String>,
    pub typedef_line: Option<u64>,
    pub byte_size: Option<u64>,
    pub base_classes: Vec<BaseClass>,
    pub is_virtual: bool,
    pub decl_file: Option<u64>, // File index from DWARF file table
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub line: Option<u64>,
    pub children: Vec<Element>,
}

/// A typedef that points to another type (not a struct/class/union/enum)
/// Used for cases like `typedef mpVector2 CharaPoint;`
#[derive(Debug, Clone)]
pub struct TypedefAlias {
    pub name: String,
    pub target_type: TypeInfo,
    pub line: Option<u64>,
    pub decl_file: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Compound(Compound),
    Function(Function),
    Variable(Variable),
    Namespace(Namespace),
    TypedefAlias(TypedefAlias),
}

#[derive(Debug)]
pub struct CompileUnit {
    pub name: String,
    pub producer: Option<String>,
    pub elements: Vec<Element>,
    pub file_table: Vec<String>, // DWARF file table (index 0 is the compile unit file)
}
