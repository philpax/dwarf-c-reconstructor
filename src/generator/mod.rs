//! Code generation implementation
//!
//! This module provides functionality to generate C/C++ code from parsed DWARF
//! type definitions and function signatures.

mod compound_gen;
mod function_gen;
mod output;
mod type_formatter;

use crate::types::*;
use std::collections::HashMap;

use compound_gen::CompoundGenerator;
use function_gen::FunctionGenerator;
use output::OutputWriter;
use type_formatter::TypeFormatter;

/// Configuration for code generation
#[derive(Clone)]
pub struct CodeGenConfig {
    #[allow(dead_code)]
    pub shorten_int_types: bool,
    pub no_function_addresses: bool,
    pub no_offsets: bool,
    pub no_function_prototypes: bool,
    pub pointer_size: u64, // 4 for 32-bit, 8 for 64-bit
    pub disable_no_line_comment: bool,
    pub verbose_class_usage: bool, // Include "class " prefix in type references (C mode only)
    pub code_style: String,        // "c" or "c++": controls which type prefixes to strip
    pub skip_namespace_indentation: bool, // Don't indent content inside namespaces
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        CodeGenConfig {
            shorten_int_types: false,
            no_function_addresses: false,
            no_offsets: false,
            no_function_prototypes: false,
            pointer_size: 4, // Default to 32-bit for backwards compatibility
            disable_no_line_comment: false,
            verbose_class_usage: false, // Don't include "class " prefix by default (C mode)
            code_style: "c".to_string(), // Default to C style (keep struct/union/enum prefixes)
            skip_namespace_indentation: false, // Indent namespace content by default
        }
    }
}

/// Main code generator
pub struct CodeGenerator {
    output: OutputWriter,
    type_sizes: HashMap<String, u64>,
    config: CodeGenConfig,
}

impl CodeGenerator {
    #[allow(dead_code)]
    pub fn with_type_sizes(type_sizes: HashMap<String, u64>) -> Self {
        CodeGenerator {
            output: OutputWriter::new(),
            type_sizes,
            config: CodeGenConfig::default(),
        }
    }

    pub fn with_config(type_sizes: HashMap<String, u64>, config: CodeGenConfig) -> Self {
        CodeGenerator {
            output: OutputWriter::new(),
            type_sizes,
            config,
        }
    }

    /// Collect type sizes from elements (for size estimation)
    pub fn collect_type_sizes_from_elements(
        type_sizes: &mut HashMap<String, u64>,
        elements: &[Element],
    ) {
        for element in elements {
            match element {
                Element::Compound(c) => {
                    if let (Some(name), Some(size)) = (&c.name, c.byte_size) {
                        // Store both with and without compound type prefix for lookup
                        type_sizes.insert(name.clone(), size);
                        type_sizes.insert(format!("{} {}", c.compound_type, name), size);
                    }
                    // Also handle typedefs
                    if let (Some(typedef_name), Some(size)) = (&c.typedef_name, c.byte_size) {
                        type_sizes.insert(typedef_name.clone(), size);
                    }
                    // Also collect sizes from nested types
                    Self::collect_type_sizes_from_compounds(type_sizes, &c.nested_types);
                }
                Element::Namespace(ns) => {
                    Self::collect_type_sizes_from_elements(type_sizes, &ns.children);
                }
                _ => {}
            }
        }
    }

    fn collect_type_sizes_from_compounds(
        type_sizes: &mut HashMap<String, u64>,
        compounds: &[Compound],
    ) {
        for c in compounds {
            if let (Some(name), Some(size)) = (&c.name, c.byte_size) {
                type_sizes.insert(name.clone(), size);
                type_sizes.insert(format!("{} {}", c.compound_type, name), size);
            }
            if let (Some(typedef_name), Some(size)) = (&c.typedef_name, c.byte_size) {
                type_sizes.insert(typedef_name.clone(), size);
            }
            Self::collect_type_sizes_from_compounds(type_sizes, &c.nested_types);
        }
    }

    /// Generate code for a compile unit
    pub fn generate_compile_unit(&mut self, cu: &CompileUnit) {
        self.output.write_line_comment("", &cu.name);
        if let Some(ref producer) = cu.producer {
            self.output
                .write_line(&format!("// Compiler: {}", producer));
        }
        self.output.push_newline();

        // Sort elements by line number
        let mut sorted_elements: Vec<(usize, &Element)> = cu.elements.iter().enumerate().collect();
        sorted_elements.sort_by_key(|(idx, elem)| {
            let line = match elem {
                Element::Compound(c) => c.typedef_line.or(c.line),
                Element::Function(f) => f.line,
                Element::Variable(v) => v.line,
                Element::Namespace(ns) => ns.line,
                Element::TypedefAlias(t) => t.line,
            };
            (line, *idx)
        });

        for (_, element) in sorted_elements {
            self.generate_element(element);
            self.output.push_newline();
        }

        self.output.write_line_comment("", &cu.name);
    }

    /// Generate a simple header file comment (for merged headers)
    pub fn generate_header_comment_simple(&mut self, header_path: &str) {
        self.output.write_line_comment("", header_path);
        self.output.push_newline();
    }

    /// Generate a source file with specific elements
    pub fn generate_source_file(
        &mut self,
        cu_name: &str,
        producer: Option<&str>,
        elements: &[&Element],
    ) {
        self.output.write_line_comment("", cu_name);
        if let Some(prod) = producer {
            self.output.write_line(&format!("// Compiler: {}", prod));
        }
        self.output.push_newline();

        self.generate_elements(elements);

        self.output.write_line_comment("", cu_name);
    }

    /// Generate elements (for header files or filtered source files)
    pub fn generate_elements(&mut self, elements: &[&Element]) {
        // Sort elements by line number
        let mut sorted_elements: Vec<(usize, &Element)> = elements
            .iter()
            .enumerate()
            .map(|(idx, &elem)| (idx, elem))
            .collect();
        sorted_elements.sort_by_key(|(idx, elem)| {
            let line = match elem {
                Element::Compound(c) => c.typedef_line.or(c.line),
                Element::Function(f) => f.line,
                Element::Variable(v) => v.line,
                Element::Namespace(ns) => ns.line,
                Element::TypedefAlias(t) => t.line,
            };
            (line, *idx)
        });

        for (_, element) in sorted_elements {
            self.generate_element(element);
            self.output.push_newline();
        }
    }

    /// Generate a single element
    fn generate_element(&mut self, element: &Element) {
        match element {
            Element::Compound(c) => {
                let mut compound_gen =
                    CompoundGenerator::new(&mut self.output, &self.config, &self.type_sizes);
                compound_gen.generate(c, &self.type_sizes);
            }
            Element::Function(f) => {
                let mut func_gen =
                    FunctionGenerator::new(&mut self.output, &self.config, &self.type_sizes);
                func_gen.generate_function(f);
            }
            Element::Variable(v) => self.generate_global_variable(v),
            Element::Namespace(ns) => self.generate_namespace(ns),
            Element::TypedefAlias(t) => self.generate_typedef_alias(t),
        }
    }

    /// Generate a namespace
    fn generate_namespace(&mut self, ns: &Namespace) {
        let line_comment = ns.line.map(|l| format!("//{}", l)).unwrap_or_default();
        self.output
            .write_line(&format!("namespace {} {{ {}", ns.name, line_comment));
        if !self.config.skip_namespace_indentation {
            self.output.indent();
        } else {
            // Add empty line after opening bracket when skipping indentation
            self.output.push_newline();
        }

        // Sort namespace children by line number
        let mut sorted_children: Vec<(usize, &Element)> = ns.children.iter().enumerate().collect();
        sorted_children.sort_by_key(|(idx, elem)| {
            let line = match elem {
                Element::Compound(c) => c.typedef_line.or(c.line),
                Element::Function(f) => f.line,
                Element::Variable(v) => v.line,
                Element::Namespace(ns) => ns.line,
                Element::TypedefAlias(t) => t.line,
            };
            (line, *idx)
        });

        for (i, (_, child)) in sorted_children.iter().enumerate() {
            if i > 0 {
                self.output.push_newline();
            }
            self.generate_element(child);
        }

        if !self.config.skip_namespace_indentation {
            self.output.dedent();
        } else {
            // Add empty line before closing bracket when skipping indentation
            self.output.push_newline();
        }
        self.output.write_line(&format!("}} //{}", ns.name));
    }

    /// Generate a global variable declaration
    fn generate_global_variable(&mut self, var: &Variable) {
        let formatter = TypeFormatter::new(&self.config, &self.type_sizes);
        let mut decl = formatter.format_type_string(&var.type_info, &var.name);

        // Add const value if present
        if let Some(ref const_val) = var.const_value {
            decl.push_str(" = ");
            match const_val {
                ConstValue::Signed(v) => decl.push_str(&v.to_string()),
                ConstValue::Unsigned(v) => decl.push_str(&v.to_string()),
            }
        }

        let line_comment = var.line.map(|l| format!(" //{}", l)).unwrap_or_default();
        self.output
            .write_line(&format!("{};{}", decl, line_comment));
    }

    /// Generate a typedef alias
    fn generate_typedef_alias(&mut self, typedef_alias: &TypedefAlias) {
        let formatter = TypeFormatter::new(&self.config, &self.type_sizes);
        let type_str =
            formatter.format_type_string(&typedef_alias.target_type, &typedef_alias.name);
        let mut decl = format!("typedef {}", type_str);
        decl.push(';');

        if let Some(line) = typedef_alias.line {
            decl.push_str(&format!(" //{}", line));
        }

        self.output.write_line(&decl);
    }

    /// Get the accumulated output
    pub fn get_output(self) -> String {
        self.output.get_output()
    }
}
