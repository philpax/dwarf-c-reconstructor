//! Code generation implementation

use crate::types::*;
use cpp_demangle::Symbol;
use std::collections::HashMap;

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

pub struct CodeGenerator {
    output: String,
    indent_level: usize,
    type_sizes: HashMap<String, u64>,
    config: CodeGenConfig,
}

impl CodeGenerator {
    #[allow(dead_code)]
    pub fn with_type_sizes(type_sizes: HashMap<String, u64>) -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
            type_sizes,
            config: CodeGenConfig::default(),
        }
    }

    pub fn with_config(type_sizes: HashMap<String, u64>, config: CodeGenConfig) -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
            type_sizes,
            config,
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    #[allow(dead_code)]
    fn shorten_type_name(&self, type_name: &str) -> String {
        if !self.config.shorten_int_types {
            return type_name.to_string();
        }

        // Apply type shortening rules
        match type_name {
            "short int" | "signed short int" | "short signed int" => "short".to_string(),
            "short unsigned int" | "unsigned short int" => "unsigned short".to_string(),
            "long int" | "signed long int" | "long signed int" => "long".to_string(),
            "long unsigned int" | "unsigned long int" => "unsigned long".to_string(),
            "long long int" | "signed long long int" | "long long signed int" => {
                "long long".to_string()
            }
            "long long unsigned int" | "unsigned long long int" => "unsigned long long".to_string(),
            "signed int" => "int".to_string(),
            _ => type_name.to_string(),
        }
    }

    /// Strip type prefixes based on code_style and verbose_class_usage settings.
    ///
    /// - C style (default): Only strip "class " prefix, keep struct/union/enum.
    ///   If verbose_class_usage is true, keep "class " prefix too.
    /// - C++ style: Strip all prefixes (class/struct/union/enum), ignoring verbose_class_usage.
    fn strip_compound_prefix(&self, type_name: &str) -> String {
        if self.config.code_style == "c++" {
            // C++ style: strip all compound type prefixes
            for prefix in &["class ", "struct ", "union ", "enum "] {
                if let Some(stripped) = type_name.strip_prefix(prefix) {
                    return stripped.to_string();
                }
            }
        } else {
            // C style (default): only strip "class " prefix, unless verbose_class_usage is set
            if !self.config.verbose_class_usage {
                if let Some(stripped) = type_name.strip_prefix("class ") {
                    return stripped.to_string();
                }
            }
            // Keep struct/union/enum prefixes in C style
        }

        type_name.to_string()
    }

    /// Apply type transformations: shorten int types and/or strip compound prefixes
    fn transform_type_name(&self, type_name: &str) -> String {
        let mut result = type_name.to_string();

        // First strip compound prefixes (unless verbose_class_usage is enabled)
        result = self.strip_compound_prefix(&result);

        // Then apply int type shortening if enabled
        if self.config.shorten_int_types {
            result = self.shorten_type_name(&result);
        }

        result
    }

    fn format_type_string(&self, type_info: &TypeInfo, var_name: &str) -> String {
        // Clone type_info and transform the base type
        let mut transformed_type = type_info.clone();
        transformed_type.base_type = self.transform_type_name(&type_info.base_type);

        // For function pointers, also transform return type and parameter types
        if transformed_type.is_function_pointer {
            if let Some(ref mut ret_type) = transformed_type.function_return_type {
                ret_type.base_type = self.transform_type_name(&ret_type.base_type);
            }
            for param in &mut transformed_type.function_params {
                param.base_type = self.transform_type_name(&param.base_type);
            }
        }

        transformed_type.to_string(var_name)
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

    fn estimate_type_size(&self, type_info: &TypeInfo) -> u64 {
        let ptr_size = self.config.pointer_size;

        // Determine the base element size
        let base_size = if type_info.pointer_count > 0 || type_info.is_function_pointer {
            // Pointers use architecture-specific size
            ptr_size
        } else {
            // Calculate size based on base type
            match type_info.base_type.as_str() {
                "char" | "unsigned char" | "signed char" | "bool" | "boolean" => 1,
                "short" | "short int" | "unsigned short" | "signed short"
                | "short unsigned int" => 2,
                "int" | "unsigned int" | "signed int" => 4,
                // long is 4 bytes on 32-bit, but can vary; use pointer size for LP64/LLP64 compat
                "long" | "unsigned long" | "signed long" | "long int" | "long unsigned int" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "long long"
                | "unsigned long long"
                | "signed long long"
                | "long long int"
                | "long long unsigned int" => 8,
                "float" => 4,
                "double" => 8,
                "long double" => {
                    if ptr_size == 8 {
                        16
                    } else {
                        12
                    }
                } // Architecture dependent
                "void" => 0,
                // For GLuint, GLint and similar types (typically typedef to unsigned int / int)
                s if s.starts_with("GL") => 4,
                // Platform-dependent types - use pointer size
                "size_t" | "ssize_t" | "ptrdiff_t" | "intptr_t" | "uintptr_t" => ptr_size,
                // Other common system types (typically 4 bytes even on 64-bit)
                "fpos_t" => ptr_size, // Often contains a pointer
                "time_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "off_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "pid_t" => 4,
                "uid_t" => 4,
                "gid_t" => 4,
                "suseconds_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "clock_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "dev_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "ino_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "mode_t" => 4,
                "nlink_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "blksize_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                "blkcnt_t" => {
                    if ptr_size == 8 {
                        8
                    } else {
                        4
                    }
                }
                // Common typedefs from various libraries
                "INT32" | "UINT32" | "DWORD" => 4,
                "INT16" | "UINT16" | "WORD" => 2,
                "INT8" | "UINT8" | "BYTE" => 1,
                "INT64" | "UINT64" | "QWORD" => 8,
                "JCOEF" => 2,      // JPEG coefficient (short)
                "JDIMENSION" => 4, // JPEG dimension (unsigned int)
                "JOCTET" => 1,     // JPEG octet (unsigned char)
                // For struct/class types, look up the byte_size from parsed types
                _ => {
                    // Look up the type size in our collected types
                    *self.type_sizes.get(&type_info.base_type).unwrap_or(&4) // Default to 4 bytes if unknown
                }
            }
        };

        // If there are arrays, multiply by total array size
        // This handles both arrays of base types and arrays of pointers
        if !type_info.array_sizes.is_empty() {
            let total_elements: usize = type_info.array_sizes.iter().product();
            return base_size * (total_elements as u64);
        }

        base_size
    }

    fn format_member_declaration(&self, var: &Variable) -> String {
        let mut decl = self.format_type_string(&var.type_info, &var.name);

        // Add bitfield specification if present
        if let Some(bit_size) = var.bit_size {
            decl.push_str(&format!(" : {}", bit_size));
        }

        // Add const value if present
        if let Some(ref const_val) = var.const_value {
            decl.push_str(" = ");
            match const_val {
                ConstValue::Signed(v) => decl.push_str(&v.to_string()),
                ConstValue::Unsigned(v) => decl.push_str(&v.to_string()),
            }
        }

        decl
    }

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
                // Store both with and without compound type prefix for lookup
                type_sizes.insert(name.clone(), size);
                type_sizes.insert(format!("{} {}", c.compound_type, name), size);
            }
            // Also handle typedefs
            if let (Some(typedef_name), Some(size)) = (&c.typedef_name, c.byte_size) {
                type_sizes.insert(typedef_name.clone(), size);
            }
            // Recursively collect sizes from nested types
            Self::collect_type_sizes_from_compounds(type_sizes, &c.nested_types);
        }
    }

    pub fn generate_compile_unit(&mut self, cu: &CompileUnit) {
        self.write_line_comment("", &cu.name);
        if let Some(ref producer) = cu.producer {
            self.write_line(&format!("// Compiler: {}", producer));
        }
        self.output.push('\n');

        // Sort elements by line number (maintain DWARF order for same line)
        let mut sorted_elements: Vec<(usize, &Element)> = cu.elements.iter().enumerate().collect();
        sorted_elements.sort_by_key(|(idx, elem)| {
            let line = match elem {
                // For compounds, use typedef_line if available (for typedef sorting)
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
            self.output.push('\n');
        }

        self.write_line_comment("", &cu.name);
    }

    /// Generate a simple header file comment (for merged headers from multiple compile units)
    pub fn generate_header_comment_simple(&mut self, header_path: &str) {
        self.write_line_comment("", header_path);
        self.output.push('\n');
    }

    /// Generate a source file with specific elements
    pub fn generate_source_file(
        &mut self,
        cu_name: &str,
        producer: Option<&str>,
        elements: &[&Element],
    ) {
        self.write_line_comment("", cu_name);
        if let Some(prod) = producer {
            self.write_line(&format!("// Compiler: {}", prod));
        }
        self.output.push('\n');

        self.generate_elements(elements);

        self.write_line_comment("", cu_name);
    }

    /// Generate elements (for header files or filtered source files)
    pub fn generate_elements(&mut self, elements: &[&Element]) {
        // Sort elements by line number (maintain DWARF order for same line)
        let mut sorted_elements: Vec<(usize, &Element)> = elements
            .iter()
            .enumerate()
            .map(|(idx, &elem)| (idx, elem))
            .collect();
        sorted_elements.sort_by_key(|(idx, elem)| {
            let line = match elem {
                // For compounds, use typedef_line if available (for typedef sorting)
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
            self.output.push('\n');
        }
    }

    fn generate_element(&mut self, element: &Element) {
        match element {
            Element::Compound(c) => self.generate_compound(c),
            Element::Function(f) => self.generate_function(f),
            Element::Variable(v) => self.generate_global_variable(v),
            Element::Namespace(ns) => self.generate_namespace(ns),
            Element::TypedefAlias(t) => self.generate_typedef_alias(t),
        }
    }

    fn generate_namespace(&mut self, ns: &Namespace) {
        let line_comment = ns.line.map(|l| format!("//{}", l)).unwrap_or_default();
        self.write_line(&format!("namespace {} {{ {}", ns.name, line_comment));
        if !self.config.skip_namespace_indentation {
            self.indent_level += 1;
        } else {
            // Add empty line after opening bracket when skipping indentation
            self.output.push('\n');
        }

        // Sort namespace children by line number (maintain DWARF order for same line)
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
                self.output.push('\n');
            }
            self.generate_element(child);
        }

        if !self.config.skip_namespace_indentation {
            self.indent_level -= 1;
        } else {
            // Add empty line before closing bracket when skipping indentation
            self.output.push('\n');
        }
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
        } else if !self.config.disable_no_line_comment {
            opening.push_str(" //No line number");
        }

        self.write_line(&opening);

        // Enum values
        self.indent_level += 1;
        for (name, value) in &compound.enum_values {
            if let Some(v) = value {
                self.write_line(&format!("{} = {}, // 0x{:x}", name, v, v));
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

        // Add size comment
        if let Some(size) = compound.byte_size {
            closing.push_str(&format!(" // sizeof: {}", size));
        }

        self.write_line(&closing);
    }

    fn generate_struct_or_union(&mut self, compound: &Compound, use_typedef: bool) {
        if compound.members.is_empty() && compound.nested_types.is_empty() {
            // Empty struct/union - just output typedef or declaration
            // Skip unnamed forward declarations as they generate invalid C code like "struct;"
            if compound.name.is_none() && compound.typedef_name.is_none() {
                return;
            }

            let mut line = String::new();

            if use_typedef {
                line.push_str("typedef ");
            }

            line.push_str(&compound.compound_type);

            if let Some(ref name) = compound.name {
                line.push(' ');
                line.push_str(name);
            }

            if use_typedef {
                if let Some(ref tname) = compound.typedef_name {
                    line.push(' ');
                    line.push_str(tname);
                }
            }

            line.push(';');

            if let Some(line_num) = compound.typedef_line.or(compound.line) {
                line.push_str(&format!(" //{}", line_num));
            } else if !self.config.disable_no_line_comment {
                line.push_str(" //No line number");
            }

            // Add size comment
            if let Some(size) = compound.byte_size {
                line.push_str(&format!(" // sizeof: {}", size));
            }

            self.write_line(&line);
        } else {
            // Struct/union with members or nested types
            let mut opening = String::new();

            if use_typedef {
                opening.push_str("typedef ");
            }

            opening.push_str(&compound.compound_type);

            if let Some(ref name) = compound.name {
                opening.push(' ');
                opening.push_str(name);
            }

            // Add base classes if any (structs can have inheritance in C++)
            if !compound.base_classes.is_empty() {
                opening.push_str(" : ");
                let bases: Vec<String> = compound
                    .base_classes
                    .iter()
                    .map(|base| {
                        let mut base_str = String::new();

                        // Add accessibility (defaults to public for structs)
                        if let Some(ref access) = base.accessibility {
                            base_str.push_str(access);
                            base_str.push(' ');
                        }

                        // Add virtual keyword if virtual inheritance
                        if base.is_virtual {
                            base_str.push_str("virtual ");
                        }

                        // Always strip "class " prefix in inheritance (never needed syntactically)
                        let base_type = base
                            .type_name
                            .strip_prefix("class ")
                            .unwrap_or(&base.type_name);
                        base_str.push_str(base_type);

                        // Add offset comment if available
                        if let Some(offset) = base.offset {
                            base_str.push_str(&format!(" /* @ offset {} */", offset));
                        }

                        base_str
                    })
                    .collect();
                opening.push_str(&bases.join(", "));
            }

            opening.push_str(" {");

            if let Some(line) = compound.line {
                opening.push_str(&format!(" //{}", line));
            } else if !self.config.disable_no_line_comment {
                opening.push_str(" //No line number");
            }

            self.write_line(&opening);

            self.indent_level += 1;

            // Generate nested types first (they're typically declared at the top)
            for nested in &compound.nested_types {
                self.generate_compound(nested);
                self.output.push('\n');
            }

            // Members grouped by line
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

            // Add size comment
            if let Some(size) = compound.byte_size {
                closing.push_str(&format!(" // sizeof: {}", size));
            }

            self.write_line(&closing);
        }
    }

    fn generate_class(&mut self, compound: &Compound) {
        // Check if this is a forward declaration (no members, no methods, no base classes, no nested types)
        let is_forward_decl = compound.members.is_empty()
            && compound.methods.is_empty()
            && compound.base_classes.is_empty()
            && compound.nested_types.is_empty();

        if is_forward_decl {
            // Forward declaration - just output "class ClassName;"
            let mut line = format!(
                "class {};",
                compound.name.as_ref().unwrap_or(&String::from("unnamed"))
            );

            if let Some(line_num) = compound.line {
                line.push_str(&format!(" //{}", line_num));
            }

            self.write_line(&line);
            return;
        }

        let mut opening = format!(
            "class {}",
            compound.name.as_ref().unwrap_or(&String::from("unnamed"))
        );

        // Add base classes if any
        if !compound.base_classes.is_empty() {
            opening.push_str(" : ");
            let bases: Vec<String> = compound
                .base_classes
                .iter()
                .map(|base| {
                    let mut base_str = String::new();

                    // Add accessibility (defaults to private for classes)
                    if let Some(ref access) = base.accessibility {
                        base_str.push_str(access);
                        base_str.push(' ');
                    }

                    // Add virtual keyword if virtual inheritance
                    if base.is_virtual {
                        base_str.push_str("virtual ");
                    }

                    // Always strip "class " prefix in inheritance (never needed syntactically)
                    let base_type = base
                        .type_name
                        .strip_prefix("class ")
                        .unwrap_or(&base.type_name);
                    base_str.push_str(base_type);

                    // Add offset comment if available
                    if let Some(offset) = base.offset {
                        base_str.push_str(&format!(" /* @ offset {} */", offset));
                    }

                    base_str
                })
                .collect();
            opening.push_str(&bases.join(", "));
        }

        opening.push_str(" {");

        if let Some(line) = compound.line {
            opening.push_str(&format!(" //{}", line));
        }

        self.write_line(&opening);

        // Generate nested types first (they're typically declared at the top of the class)
        // Nested types default to private accessibility in classes
        if !compound.nested_types.is_empty() {
            self.indent_level += 1;
            for nested in &compound.nested_types {
                self.generate_compound(nested);
                self.output.push('\n');
            }
            self.indent_level -= 1;
        }

        // Group members and methods by accessibility
        // Default accessibility: public (when DW_AT_accessibility is not present)
        // Values: 1=public, 2=protected, 3=private

        let mut public_members: Vec<&Variable> = Vec::new();
        let mut protected_members: Vec<&Variable> = Vec::new();
        let mut private_members: Vec<&Variable> = Vec::new();

        for member in &compound.members {
            match member.accessibility.as_deref() {
                Some("protected") => protected_members.push(member),
                Some("public") => public_members.push(member),
                Some("private") => private_members.push(member),
                // No accessibility specified or unknown - default to public
                None | Some(_) => public_members.push(member),
            }
        }

        let mut public_methods: Vec<&Function> = Vec::new();
        let mut protected_methods: Vec<&Function> = Vec::new();
        let mut private_methods: Vec<&Function> = Vec::new();

        for method in &compound.methods {
            match method.accessibility.as_deref() {
                Some("protected") => protected_methods.push(method),
                Some("public") => public_methods.push(method),
                Some("private") => private_methods.push(method),
                // No accessibility specified or unknown - default to public
                None | Some(_) => public_methods.push(method),
            }
        }

        // Write sections - access specifiers at same indent level as class
        // Order: public, protected, private (conventional C++ style)
        if !public_members.is_empty() || !public_methods.is_empty() {
            self.write_line("public:");
            self.indent_level += 1;
            self.generate_members(public_members.to_vec().as_slice());
            // Sort methods by line number
            let mut sorted_methods: Vec<(usize, &Function)> = public_methods
                .iter()
                .enumerate()
                .map(|(i, &f)| (i, f))
                .collect();
            sorted_methods.sort_by_key(|(idx, m)| (m.line, *idx));
            for (_, method) in sorted_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !protected_members.is_empty() || !protected_methods.is_empty() {
            self.write_line("protected:");
            self.indent_level += 1;
            self.generate_members(protected_members.to_vec().as_slice());
            // Sort methods by line number
            let mut sorted_methods: Vec<(usize, &Function)> = protected_methods
                .iter()
                .enumerate()
                .map(|(i, &f)| (i, f))
                .collect();
            sorted_methods.sort_by_key(|(idx, m)| (m.line, *idx));
            for (_, method) in sorted_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !private_members.is_empty() || !private_methods.is_empty() {
            self.write_line("private:");
            self.indent_level += 1;
            self.generate_members(private_members.to_vec().as_slice());
            // Sort methods by line number
            let mut sorted_methods: Vec<(usize, &Function)> = private_methods
                .iter()
                .enumerate()
                .map(|(i, &f)| (i, f))
                .collect();
            sorted_methods.sort_by_key(|(idx, m)| (m.line, *idx));
            for (_, method) in sorted_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        let mut closing = String::from("};");
        // Add size and vtable comments
        if let Some(size) = compound.byte_size {
            closing.push_str(&format!(" // sizeof: {}", size));
        }
        if compound.is_virtual {
            closing.push_str(" [has vtable]");
        }
        self.write_line(&closing);
    }

    fn generate_members(&mut self, members: &[&Variable]) {
        // Check if we have offset information for members
        let has_any_offsets = members.iter().any(|m| m.offset.is_some());

        if has_any_offsets {
            // Group members by line number, sorting by offset within each line
            let mut lines: HashMap<Option<u64>, Vec<(&Variable, u64)>> = HashMap::new();
            let mut no_offset_vars = Vec::new();

            for member in members {
                if let Some(offset) = member.offset {
                    lines.entry(member.line).or_default().push((member, offset));
                } else {
                    no_offset_vars.push(*member);
                }
            }

            // Sort each line's members by offset
            for vars in lines.values_mut() {
                vars.sort_by_key(|(_, offset)| *offset);
            }

            // Sort lines by their first member's offset (for padding detection)
            let mut sorted_lines: Vec<_> = lines.iter().collect();
            sorted_lines.sort_by_key(|(_, vars)| vars.first().map(|(_, o)| *o).unwrap_or(0));

            let mut prev_end_offset: Option<u64> = None;

            for (line, vars) in sorted_lines {
                if vars.is_empty() {
                    continue;
                }

                let first_offset = vars[0].1;
                let last_var = vars.last().unwrap().0;
                let last_offset = vars.last().unwrap().1;
                let last_member_size = self.estimate_type_size(&last_var.type_info);

                // Detect padding before this group
                if let Some(prev_end) = prev_end_offset {
                    if first_offset > prev_end {
                        let padding_bytes = first_offset - prev_end;
                        self.write_line(&format!(
                            "// [{} byte{} padding for alignment]",
                            padding_bytes,
                            if padding_bytes == 1 { "" } else { "s" }
                        ));
                    }
                }

                // Group by type compatibility within this line
                let mut type_groups: Vec<Vec<(&Variable, u64)>> = Vec::new();

                for (var, offset) in vars {
                    // Bitfields can't be grouped with other variables
                    if var.bit_size.is_some() {
                        type_groups.push(vec![(var, *offset)]);
                        continue;
                    }

                    let mut added = false;
                    for group in &mut type_groups {
                        // Don't group with bitfields
                        if group[0].0.bit_size.is_some() {
                            continue;
                        }
                        if self.types_compatible(&group[0].0.type_info, &var.type_info) {
                            group.push((var, *offset));
                            added = true;
                            break;
                        }
                    }
                    if !added {
                        type_groups.push(vec![(var, *offset)]);
                    }
                }

                // Generate declarations for this line
                let mut decls = Vec::new();

                for group in &type_groups {
                    // Check if this group contains function pointers or bitfields
                    if group[0].0.type_info.is_function_pointer || group[0].0.bit_size.is_some() {
                        // Output individually with bitfield info
                        for (var, offset) in group {
                            let mut decl = self.format_member_declaration(var);
                            if let (Some(_bit_size), Some(bit_offset)) =
                                (var.bit_size, var.bit_offset)
                            {
                                decl.push_str(&format!(" [bit offset: {}]", bit_offset));
                            }
                            // Store offset for later
                            decls.push((decl, Some(*offset)));
                        }
                    } else {
                        let base_type = &group[0].0.type_info;
                        let mut var_names = Vec::new();

                        for (var, _) in group {
                            let ptr_str = "*".repeat(var.type_info.pointer_count);
                            let mut name_with_array = format!("{}{}", ptr_str, var.name);

                            for size in &var.type_info.array_sizes {
                                name_with_array.push_str(&format!("[{}]", size));
                            }

                            var_names.push(name_with_array);
                        }

                        let transformed_base = self.transform_type_name(&base_type.base_type);
                        let mut decl = transformed_base;
                        decl.push(' ');
                        decl.push_str(&var_names.join(", "));
                        decls.push((decl, Some(group[0].1)));
                    }
                }

                // Output all declarations for this line
                for (decl, offset) in decls {
                    let mut full_decl = decl;
                    full_decl.push(';');

                    if let Some(l) = line {
                        full_decl.push_str(&format!(" //{}", l));
                    }
                    if !self.config.no_offsets {
                        if let Some(o) = offset {
                            full_decl.push_str(&format!(" @ offset {}", o));
                        }
                    }
                    self.write_line(&full_decl);
                }

                // Update prev_end_offset based on last member in this line group
                prev_end_offset = Some(last_offset + last_member_size);
            }

            // Output members without offsets at the end
            for var in no_offset_vars {
                let decl = self.format_member_declaration(var);
                if let Some(line) = var.line {
                    self.write_line_comment(&format!("{};", decl), &line.to_string());
                } else {
                    self.write_line(&format!("{};", decl));
                }
            }
        } else {
            // No offset information - use original line-based grouping
            let mut lines: HashMap<u64, Vec<&Variable>> = HashMap::new();
            let mut no_line_vars = Vec::new();

            for member in members {
                if let Some(line) = member.line {
                    lines.entry(line).or_default().push(member);
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
                    // Bitfields can't be grouped with other variables
                    if var.bit_size.is_some() {
                        type_groups.push(vec![var]);
                        continue;
                    }

                    let mut added = false;
                    for group in &mut type_groups {
                        // Don't group with bitfields
                        if group[0].bit_size.is_some() {
                            continue;
                        }
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
                    // Check if this group contains function pointers or bitfields - they can't be grouped
                    if group[0].type_info.is_function_pointer || group[0].bit_size.is_some() {
                        // Output individually
                        for var in group {
                            let decl = self.format_member_declaration(var);
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

                        let transformed_base = self.transform_type_name(&base_type.base_type);
                        let mut decl = transformed_base;
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
                let decl = self.format_member_declaration(var);
                self.write_line(&format!("{};", decl));
            }
        }
    }

    fn types_compatible(&self, t1: &TypeInfo, t2: &TypeInfo) -> bool {
        // Two types are compatible for joining if they have the same base type
        // and differ only in pointer count or array sizes
        t1.base_type == t2.base_type && !t1.is_function_pointer && !t2.is_function_pointer
    }

    fn generate_method(&mut self, func: &Function) {
        // Inside a class definition, always output declaration-only format
        // Method definitions are output as top-level functions with ClassName:: prefix
        let decl = self.generate_method_declaration(func);
        self.write_line(&decl);
    }

    fn generate_method_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Detect constructor: name matches class name
        let is_constructor = func.class_name.as_ref() == Some(&func.name);

        // Virtual keyword
        if func.is_virtual {
            decl.push_str("virtual ");
        }

        // Return type (skip for constructors/destructors, apply type transformations)
        // For return types, pointer/reference stays with the type (e.g., int* func())
        if !is_constructor && !func.is_destructor {
            decl.push_str(&self.transform_type_name(&func.return_type.base_type));
            if func.return_type.is_rvalue_reference {
                decl.push_str("&&");
            } else if func.return_type.is_reference {
                decl.push('&');
            } else if func.return_type.pointer_count > 0 {
                decl.push_str(&"*".repeat(func.return_type.pointer_count));
            }
            decl.push(' ');
        }

        // Function name
        decl.push_str(&func.name);
        decl.push('(');

        // Parameters - filter out:
        // 1. 'this' without line number for methods (implicit this pointer)
        // 2. Compiler-generated parameters (e.g., __in_chrg for destructors, __vtt_parm)
        let params: Vec<_> = func
            .parameters
            .iter()
            .filter(|p| {
                // Skip 'this' without line number
                if p.name == "this" && p.line.is_none() {
                    return false;
                }
                // Skip compiler-generated parameters (names starting with __)
                if p.name.starts_with("__") && p.line.is_none() {
                    return false;
                }
                true
            })
            .collect();

        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                decl.push_str(", ");
            }
            decl.push_str(&self.format_type_string(&param.type_info, &param.name));
        }
        decl.push_str(");");

        // Add line comment after semicolon
        if let Some(line) = func.line {
            decl.push_str(&format!(" //{}", line));
        }

        // Add metadata comment (mangled name, artificial flag)
        let metadata = self.generate_function_metadata_comment(func);
        if !metadata.is_empty() {
            if func.line.is_none() {
                decl.push_str(" //");
            }
            decl.push(' ');
            decl.push_str(&metadata);
        }

        decl
    }

    fn generate_function_metadata_comment(&self, func: &Function) -> String {
        let mut comment = String::new();

        // Add linkage name (demangle C++ names if possible, otherwise show raw) if not disabled
        if !self.config.no_function_prototypes {
            if let Some(ref linkage_name) = func.linkage_name {
                if !comment.is_empty() {
                    comment.push(' ');
                }

                // Try to demangle C++ symbols
                if let Ok(sym) = Symbol::new(linkage_name.as_bytes()) {
                    comment.push_str(&format!("[{}]", sym));
                } else {
                    // Not a C++ mangled name (e.g., C function), show as-is
                    comment.push_str(&format!("[{}]", linkage_name));
                }
            }
        }

        // Add artificial flag if it's compiler-generated
        if func.is_artificial {
            if !comment.is_empty() {
                comment.push(' ');
            }
            comment.push_str("[compiler-generated]");
        }

        comment
    }

    fn insert_semicolon_before_comment(&self, decl: &str) -> String {
        // Find the position of the first comment marker "//"
        // We need to insert the semicolon before it
        if let Some(comment_pos) = decl.rfind(" //") {
            let mut result = String::new();
            result.push_str(&decl[..comment_pos]);
            result.push(';');
            result.push_str(&decl[comment_pos..]);
            result
        } else {
            // No comment found, just add semicolon at the end
            format!("{};", decl)
        }
    }

    fn generate_function(&mut self, func: &Function) {
        // If this is a method definition with a namespace path, wrap it in namespace blocks
        if func.is_method && !func.namespace_path.is_empty() {
            self.generate_function_with_namespace(func);
        } else {
            self.generate_function_impl(func);
        }
    }

    /// Generate a function wrapped in namespace blocks
    fn generate_function_with_namespace(&mut self, func: &Function) {
        // Open namespace blocks
        for ns in &func.namespace_path {
            self.write_line(&format!("namespace {} {{", ns));
            if !self.config.skip_namespace_indentation {
                self.indent_level += 1;
            } else {
                // Add empty line after opening bracket when skipping indentation
                self.output.push('\n');
            }
        }

        // Generate the function
        self.generate_function_impl(func);

        // Close namespace blocks in reverse order
        for ns in func.namespace_path.iter().rev() {
            if !self.config.skip_namespace_indentation {
                self.indent_level -= 1;
            } else {
                // Add empty line before closing bracket when skipping indentation
                self.output.push('\n');
            }
            self.write_line(&format!("}} //{}", ns));
        }
    }

    fn generate_function_impl(&mut self, func: &Function) {
        // Write address comment above function if available and not disabled
        if !self.config.no_function_addresses {
            if let (Some(low), Some(high)) = (func.low_pc, func.high_pc) {
                let size = high.saturating_sub(low);
                self.write_line(&format!("// @ 0x{:x}-0x{:x} ({} bytes)", low, high, size));
            }
        }

        let decl = self.generate_function_declaration(func);

        if !func.has_body
            || (func.variables.is_empty()
                && func.lexical_blocks.is_empty()
                && func.inlined_calls.is_empty()
                && func.labels.is_empty())
        {
            // Function declaration only - add semicolon before line comment
            let decl_with_semicolon = self.insert_semicolon_before_comment(&decl);
            self.write_line(&decl_with_semicolon);
        } else {
            self.write_line(&decl);

            // Check if we have a single lexical block at top level with no other variables/inlined calls
            // In this case, we output the block contents directly without extra braces,
            // since the lexical block provides the necessary braces
            let single_block_only = func.lexical_blocks.len() == 1
                && func.variables.is_empty()
                && func.inlined_calls.is_empty();

            if single_block_only {
                // Use function braces to contain the block's content (without the lexical block's own braces)
                self.write_line("{");
                self.indent_level += 1;

                // Set up label queue for interleaving
                let mut pending_labels: std::collections::VecDeque<&Label> =
                    func.labels.iter().collect::<Vec<_>>().into_iter().collect();
                pending_labels
                    .make_contiguous()
                    .sort_by_key(|l| (l.line, l.name.as_str()));

                // Output the lexical block's contents directly (without its own braces)
                self.generate_lexical_block_contents_with_labels(
                    &func.lexical_blocks[0],
                    &mut pending_labels,
                );

                // Output any remaining labels at the end
                while let Some(label) = pending_labels.pop_front() {
                    let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
                    self.output
                        .push_str(&format!("{}:{}\n", label.name, line_comment));
                }

                self.indent_level -= 1;
                self.write_line("}");
            } else {
                // Add our own braces
                self.write_line("{");
                self.indent_level += 1;
                self.generate_function_body(func);
                self.indent_level -= 1;
                self.write_line("}")
            }
        }
    }

    fn generate_function_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Static/extern specifier (only for non-method functions)
        if !func.is_method && !func.is_external {
            decl.push_str("static ");
        }
        // Note: "extern" is implicit for external functions, so we don't output it

        // Inline specifier
        if func.is_inline {
            decl.push_str("inline ");
        }

        // Detect constructor: name matches class name
        let is_constructor = func.class_name.as_ref() == Some(&func.name);

        // Return type (skip for constructors/destructors, apply type transformations)
        // For return types, pointer/reference stays with the type (e.g., int* func())
        if !is_constructor && !func.is_destructor {
            decl.push_str(&self.transform_type_name(&func.return_type.base_type));
            if func.return_type.is_rvalue_reference {
                decl.push_str("&&");
            } else if func.return_type.is_reference {
                decl.push('&');
            } else if func.return_type.pointer_count > 0 {
                decl.push_str(&"*".repeat(func.return_type.pointer_count));
            }
            decl.push(' ');
        }

        // Function name (with class prefix for method definitions)
        if func.is_method {
            if let Some(ref class_name) = func.class_name {
                decl.push_str(class_name);
                decl.push_str("::");
            }
        }
        decl.push_str(&func.name);
        decl.push('(');

        // Parameters - filter out:
        // 1. 'this' without line number for methods (implicit this pointer)
        // 2. Compiler-generated parameters (e.g., __in_chrg for destructors, __vtt_parm)
        let params: Vec<_> = func
            .parameters
            .iter()
            .filter(|p| {
                // Skip 'this' without line number
                if p.name == "this" && p.line.is_none() {
                    return false;
                }
                // Skip compiler-generated parameters (names starting with __)
                if p.name.starts_with("__") && p.line.is_none() {
                    return false;
                }
                true
            })
            .collect();

        if params.is_empty() {
            decl.push(')');
            if let Some(line) = func.line {
                decl.push_str(&format!(" //{}", line));
            }
            // Add metadata comment (mangled name, artificial flag)
            let metadata = self.generate_function_metadata_comment(func);
            if !metadata.is_empty() {
                if func.line.is_none() {
                    decl.push_str(" //");
                }
                decl.push(' ');
                decl.push_str(&metadata);
            }
        } else {
            // Check if all params are on same line as function
            let all_same_line = params.iter().all(|p| p.line == func.line);

            if all_same_line {
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        decl.push_str(", ");
                    }
                    decl.push_str(&self.format_type_string(&param.type_info, &param.name));
                }
                decl.push(')');
                if let Some(line) = func.line {
                    decl.push_str(&format!(" //{}", line));
                }
                // Add metadata comment (mangled name, artificial flag)
                let metadata = self.generate_function_metadata_comment(func);
                if !metadata.is_empty() {
                    if func.line.is_none() {
                        decl.push_str(" //");
                    }
                    decl.push(' ');
                    decl.push_str(&metadata);
                }
            } else {
                // Parameters on different lines
                // Group parameters by line
                let mut param_lines: HashMap<Option<u64>, Vec<&Parameter>> = HashMap::new();
                for param in &params {
                    param_lines.entry(param.line).or_default().push(param);
                }

                let mut sorted_lines: Vec<_> = param_lines.iter().collect();
                sorted_lines.sort_by_key(|(line, _)| *line);

                for (idx, (line, params_at_line)) in sorted_lines.iter().enumerate() {
                    // First group: always put on same line as function name
                    if idx == 0 {
                        // Put first params on same line as function name
                        for (i, param) in params_at_line.iter().enumerate() {
                            if i > 0 {
                                decl.push_str(", ");
                            }
                            decl.push_str(&self.format_type_string(&param.type_info, &param.name));
                        }

                        if sorted_lines.len() == 1 {
                            decl.push(')');
                        } else {
                            decl.push(',');
                        }

                        if let Some(l) = line {
                            decl.push_str(&format!(" //{}", l));
                        }

                        if sorted_lines.len() > 1 {
                            decl.push('\n');
                        }
                    } else {
                        // Subsequent groups: put on new lines with indentation
                        decl.push_str(&self.indent());
                        decl.push_str("        ");

                        for (i, param) in params_at_line.iter().enumerate() {
                            if i > 0 {
                                decl.push_str(", ");
                            }
                            decl.push_str(&self.format_type_string(&param.type_info, &param.name));
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

                    // Add metadata on the last parameter line
                    if idx == sorted_lines.len() - 1 {
                        let metadata = self.generate_function_metadata_comment(func);
                        if !metadata.is_empty() {
                            if line.is_none() {
                                decl.push_str(" //");
                            }
                            decl.push(' ');
                            decl.push_str(&metadata);
                        }
                    }
                }
            }
        }

        decl
    }

    fn generate_function_body(&mut self, func: &Function) {
        // Collect all function-level labels sorted by line number
        // These need to be interleaved with all content (including inside lexical blocks)
        let mut pending_labels: std::collections::VecDeque<&Label> =
            func.labels.iter().collect::<Vec<_>>().into_iter().collect();
        pending_labels
            .make_contiguous()
            .sort_by_key(|l| (l.line, l.name.as_str()));

        // Collect non-label elements with their indices for stable sorting
        enum BodyElement<'a> {
            Variable(&'a Variable, usize),
            InlinedCall(&'a InlinedSubroutine, usize),
            LexicalBlock(&'a LexicalBlock, usize),
        }

        let mut elements: Vec<BodyElement> = Vec::new();

        for (idx, var) in func.variables.iter().enumerate() {
            elements.push(BodyElement::Variable(var, idx));
        }
        for (idx, inlined) in func.inlined_calls.iter().enumerate() {
            elements.push(BodyElement::InlinedCall(inlined, idx));
        }
        for (idx, block) in func.lexical_blocks.iter().enumerate() {
            elements.push(BodyElement::LexicalBlock(block, idx));
        }

        // Sort by line number, then by original index for stable sort
        // For lexical blocks, use min_content_line() to sort by the earliest line in the block
        let mut keyed_elements: Vec<((Option<u64>, usize), BodyElement)> = elements
            .into_iter()
            .map(|elem| {
                let key = match &elem {
                    BodyElement::Variable(v, idx) => (v.line, *idx),
                    BodyElement::InlinedCall(i, idx) => (i.line, *idx),
                    BodyElement::LexicalBlock(bl, idx) => (bl.min_content_line(), *idx),
                };
                (key, elem)
            })
            .collect();
        keyed_elements.sort_by_key(|(key, _)| *key);

        // Generate in sorted order, grouping variables and inlined calls by line
        let mut variables_buffer: Vec<&Variable> = Vec::new();
        let mut inlined_buffer: Vec<&InlinedSubroutine> = Vec::new();
        let mut last_var_line: Option<u64> = None;
        let mut last_inlined_line: Option<u64> = None;

        for (_, element) in keyed_elements {
            match element {
                BodyElement::Variable(var, _) => {
                    // Output any pending labels that come before this variable
                    self.output_pending_labels_before(&mut pending_labels, var.line);
                    // Flush inlined calls buffer if we're switching to variables
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    // Buffer variables to group them by line
                    if last_var_line.is_some()
                        && last_var_line != var.line
                        && !variables_buffer.is_empty()
                    {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    variables_buffer.push(var);
                    last_var_line = var.line;
                }
                BodyElement::InlinedCall(inlined, _) => {
                    // Output any pending labels that come before this inlined call
                    self.output_pending_labels_before(&mut pending_labels, inlined.line);
                    // Flush any buffered variables first
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    // Buffer inlined calls to group them by line
                    if last_inlined_line.is_some()
                        && last_inlined_line != inlined.line
                        && !inlined_buffer.is_empty()
                    {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    inlined_buffer.push(inlined);
                    last_inlined_line = inlined.line;
                }
                BodyElement::LexicalBlock(block, _) => {
                    // Output any pending labels that come before this lexical block
                    // (labels with line numbers less than the block's minimum content line)
                    self.output_pending_labels_before(
                        &mut pending_labels,
                        block.min_content_line(),
                    );
                    // Flush any buffered elements first
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    // Pass remaining pending labels to the lexical block so it can interleave them
                    self.generate_lexical_block_with_labels(block, &mut pending_labels);
                }
            }
        }

        // Flush any remaining buffered elements
        if !variables_buffer.is_empty() {
            self.generate_variables(&variables_buffer);
        }
        if !inlined_buffer.is_empty() {
            self.generate_inlined_calls(&inlined_buffer);
        }

        // Output any remaining labels at the end
        while let Some(label) = pending_labels.pop_front() {
            let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.output
                .push_str(&format!("{}:{}\n", label.name, line_comment));
        }
    }

    /// Output any pending labels that should appear before the given line number
    fn output_pending_labels_before(
        &mut self,
        pending_labels: &mut std::collections::VecDeque<&Label>,
        before_line: Option<u64>,
    ) {
        while let Some(label) = pending_labels.front() {
            // Check if this label should come before the current element
            match (label.line, before_line) {
                (Some(label_line), Some(elem_line)) if label_line < elem_line => {
                    let label = pending_labels.pop_front().unwrap();
                    self.output
                        .push_str(&format!("{}: //{}\n", label.name, label_line));
                }
                (None, _) => {
                    // Labels without line numbers go at the start
                    let label = pending_labels.pop_front().unwrap();
                    self.output.push_str(&format!("{}:\n", label.name));
                }
                _ => break, // Label should not come before this element
            }
        }
    }

    fn generate_inlined_calls(&mut self, inlined_calls: &[&InlinedSubroutine]) {
        if inlined_calls.is_empty() {
            return;
        }

        // All calls in this buffer are on the same line
        let mut line = String::new();
        line.push_str(&self.indent());

        for (i, inlined) in inlined_calls.iter().enumerate() {
            if i > 0 {
                line.push(' ');
            }
            line.push_str(&inlined.name);
            line.push_str("();");
        }

        // Add line comment from the first call (they all share the same line)
        if let Some(l) = inlined_calls[0].line {
            line.push_str(&format!(" //{}", l));
        }

        self.output.push_str(&line);
        self.output.push('\n');
    }

    fn generate_variables(&mut self, variables: &[&Variable]) {
        // Group by line number and type compatibility
        let mut lines: HashMap<u64, Vec<&Variable>> = HashMap::new();
        let mut no_line_vars = Vec::new();

        for &var in variables {
            if let Some(line) = var.line {
                lines.entry(line).or_default().push(var);
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
                        let decl = self.format_type_string(&var.type_info, &var.name);
                        decls.push(decl);
                    }
                } else {
                    // Check if any variables in this group have const values
                    // If so, output them individually
                    let has_const_values = group.iter().any(|v| v.const_value.is_some());

                    if has_const_values {
                        // Output individually with const values
                        for var in group {
                            let mut decl = self.format_type_string(&var.type_info, &var.name);
                            if let Some(ref const_val) = var.const_value {
                                decl.push_str(" = ");
                                match const_val {
                                    ConstValue::Signed(v) => decl.push_str(&v.to_string()),
                                    ConstValue::Unsigned(v) => decl.push_str(&v.to_string()),
                                }
                            }
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

                        // Apply type transformations to the base type
                        let transformed_base = self.transform_type_name(&base_type.base_type);
                        let mut decl = transformed_base;
                        decl.push(' ');
                        decl.push_str(&var_names.join(", "));
                        decls.push(decl);
                    }
                }
            }

            let full_decl = decls.join("; ");
            self.write_line_comment(&format!("{};", full_decl), &line.to_string());
        }

        // Variables without line numbers
        for var in no_line_vars {
            self.write_line(&format!(
                "{};",
                self.format_type_string(&var.type_info, &var.name)
            ));
        }
    }

    fn generate_lexical_block_with_labels(
        &mut self,
        block: &LexicalBlock,
        pending_labels: &mut std::collections::VecDeque<&Label>,
    ) {
        self.write_line("{");
        self.indent_level += 1;
        self.generate_lexical_block_contents_with_labels(block, pending_labels);
        self.indent_level -= 1;
        self.write_line("}");
    }

    /// Generate the contents of a lexical block, interleaving pending labels from parent scope
    fn generate_lexical_block_contents_with_labels(
        &mut self,
        block: &LexicalBlock,
        pending_labels: &mut std::collections::VecDeque<&Label>,
    ) {
        // Collect block-level labels and merge with pending function-level labels
        let mut all_labels: std::collections::VecDeque<&Label> = block.labels.iter().collect();
        // Sort block labels
        all_labels
            .make_contiguous()
            .sort_by_key(|l| (l.line, l.name.as_str()));

        // Collect non-label elements with their indices for stable sorting
        enum BlockElement<'a> {
            Variable(&'a Variable, usize),
            InlinedCall(&'a InlinedSubroutine, usize),
            NestedBlock(&'a LexicalBlock, usize),
        }

        let mut elements: Vec<BlockElement> = Vec::new();

        for (idx, var) in block.variables.iter().enumerate() {
            elements.push(BlockElement::Variable(var, idx));
        }
        for (idx, inlined) in block.inlined_calls.iter().enumerate() {
            elements.push(BlockElement::InlinedCall(inlined, idx));
        }
        for (idx, nested) in block.nested_blocks.iter().enumerate() {
            elements.push(BlockElement::NestedBlock(nested, idx));
        }

        // Sort by line number, then by original index for stable sort
        let mut keyed_elements: Vec<((Option<u64>, usize), BlockElement)> = elements
            .into_iter()
            .map(|elem| {
                let key = match &elem {
                    BlockElement::Variable(v, idx) => (v.line, *idx),
                    BlockElement::InlinedCall(i, idx) => (i.line, *idx),
                    BlockElement::NestedBlock(bl, idx) => (bl.min_content_line(), *idx),
                };
                (key, elem)
            })
            .collect();
        keyed_elements.sort_by_key(|(key, _)| *key);

        // Generate in sorted order, grouping variables and inlined calls by line
        let mut variables_buffer: Vec<&Variable> = Vec::new();
        let mut inlined_buffer: Vec<&InlinedSubroutine> = Vec::new();
        let mut last_var_line: Option<u64> = None;
        let mut last_inlined_line: Option<u64> = None;

        for (_, element) in keyed_elements {
            match element {
                BlockElement::Variable(var, _) => {
                    // Output any pending labels (function-level and block-level) before this variable
                    self.output_pending_labels_before(pending_labels, var.line);
                    self.output_pending_labels_before(&mut all_labels, var.line);
                    // Flush inlined calls buffer if we're switching to variables
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    // Buffer variables to group them by line
                    if last_var_line.is_some()
                        && last_var_line != var.line
                        && !variables_buffer.is_empty()
                    {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    variables_buffer.push(var);
                    last_var_line = var.line;
                }
                BlockElement::InlinedCall(inlined, _) => {
                    // Output any pending labels before this inlined call
                    self.output_pending_labels_before(pending_labels, inlined.line);
                    self.output_pending_labels_before(&mut all_labels, inlined.line);
                    // Flush any buffered variables first
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    // Buffer inlined calls to group them by line
                    if last_inlined_line.is_some()
                        && last_inlined_line != inlined.line
                        && !inlined_buffer.is_empty()
                    {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    inlined_buffer.push(inlined);
                    last_inlined_line = inlined.line;
                }
                BlockElement::NestedBlock(nested, _) => {
                    // Output any pending labels that come before this nested block
                    self.output_pending_labels_before(pending_labels, nested.min_content_line());
                    self.output_pending_labels_before(&mut all_labels, nested.min_content_line());
                    // Flush any buffered elements first
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    // Pass remaining pending labels to nested block
                    self.generate_lexical_block_with_labels(nested, pending_labels);
                }
            }
        }

        // Flush any remaining buffered elements
        if !variables_buffer.is_empty() {
            self.generate_variables(&variables_buffer);
        }
        if !inlined_buffer.is_empty() {
            self.generate_inlined_calls(&inlined_buffer);
        }

        // Output any remaining block-level labels
        while let Some(label) = all_labels.pop_front() {
            let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.output
                .push_str(&format!("{}:{}\n", label.name, line_comment));
        }
    }

    fn generate_global_variable(&mut self, var: &Variable) {
        let mut decl = self.format_type_string(&var.type_info, &var.name);

        // Add const value if present
        if let Some(ref const_val) = var.const_value {
            decl.push_str(" = ");
            match const_val {
                ConstValue::Signed(v) => decl.push_str(&v.to_string()),
                ConstValue::Unsigned(v) => decl.push_str(&v.to_string()),
            }
        }

        let line_comment = var.line.map(|l| format!(" //{}", l)).unwrap_or_default();
        self.write_line(&format!("{};{}", decl, line_comment));
    }

    fn generate_typedef_alias(&mut self, typedef_alias: &TypedefAlias) {
        // Use format_type_string for correct syntax with type shortening
        // This handles arrays correctly: typedef int arr_t[10]; (not typedef int [10] arr_t;)
        let type_str = self.format_type_string(&typedef_alias.target_type, &typedef_alias.name);
        let mut decl = format!("typedef {}", type_str);
        decl.push(';');

        if let Some(line) = typedef_alias.line {
            decl.push_str(&format!(" //{}", line));
        }

        self.write_line(&decl);
    }

    pub fn get_output(self) -> String {
        self.output
    }
}
