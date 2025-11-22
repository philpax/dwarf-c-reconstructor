//! Type definitions for DWARF C reconstructor

pub type DwarfReader = gimli::EndianSlice<'static, gimli::LittleEndian>;
pub type DwarfUnit = gimli::Unit<DwarfReader>;

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
            if let Some(ret_type) = &self.function_return_type {
                // Add const for return type if present
                if ret_type.is_const {
                    result.push_str("const ");
                }
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
                    // Add const for parameter if present
                    if param.is_const {
                        result.push_str("const ");
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

            // References and pointers
            if self.is_rvalue_reference {
                result.push_str("&&");
            } else if self.is_reference {
                result.push('&');
            } else {
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
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_info: TypeInfo,
    pub line: Option<u64>,
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub line: Option<u64>,
}

#[derive(Debug)]
#[allow(dead_code)] // line field may be used in future
pub struct LexicalBlock {
    pub variables: Vec<Variable>,
    pub nested_blocks: Vec<LexicalBlock>,
    pub inlined_calls: Vec<InlinedSubroutine>,
    pub labels: Vec<Label>,
    pub line: Option<u64>,
}

#[derive(Debug)]
pub struct InlinedSubroutine {
    pub name: String,
    pub line: Option<u64>,
}

#[derive(Debug)]
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
}

#[derive(Debug, Clone)]
pub struct BaseClass {
    pub type_name: String,
    pub offset: Option<u64>,
    pub accessibility: Option<String>,
    pub is_virtual: bool,
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub line: Option<u64>,
    pub children: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
    Compound(Compound),
    Function(Function),
    Variable(Variable),
    Namespace(Namespace),
}

#[derive(Debug)]
pub struct CompileUnit {
    pub name: String,
    pub producer: Option<String>,
    pub elements: Vec<Element>,
}
