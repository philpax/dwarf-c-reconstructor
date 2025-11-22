//! Code generation implementation

use crate::types::*;
use cpp_demangle::Symbol;
use std::collections::HashMap;

pub struct CodeGenerator {
    output: String,
    indent_level: usize,
    type_sizes: HashMap<String, u64>,
}

impl CodeGenerator {
    pub fn with_type_sizes(type_sizes: HashMap<String, u64>) -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
            type_sizes,
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

    fn estimate_type_size(&self, type_info: &TypeInfo) -> u64 {
        // Determine the base element size
        let base_size = if type_info.pointer_count > 0 || type_info.is_function_pointer {
            // Pointers are 4 bytes (32-bit architecture based on sample output)
            4
        } else {
            // Calculate size based on base type
            match type_info.base_type.as_str() {
                "char" | "unsigned char" | "signed char" | "bool" => 1,
                "short" | "short int" | "unsigned short" | "signed short"
                | "short unsigned int" => 2,
                "int" | "unsigned int" | "signed int" => 4,
                "long" | "unsigned long" | "signed long" | "long int" | "long unsigned int" => 4, // 32-bit long
                "long long"
                | "unsigned long long"
                | "signed long long"
                | "long long int"
                | "long long unsigned int" => 8,
                "float" => 4,
                "double" => 8,
                "long double" => 12, // x86 extended precision
                "void" => 0,
                // For GLuint, GLint and similar types (typically typedef to unsigned int / int)
                s if s.starts_with("GL") => 4,
                // Common system types (32-bit)
                "fpos_t" => 4,
                "time_t" => 4,
                "size_t" => 4,
                "ssize_t" => 4,
                "off_t" => 4,
                "pid_t" => 4,
                "uid_t" => 4,
                "gid_t" => 4,
                "suseconds_t" => 4,
                "clock_t" => 4,
                "dev_t" => 4,
                "ino_t" => 4,
                "mode_t" => 4,
                "nlink_t" => 4,
                "blksize_t" => 4,
                "blkcnt_t" => 4,
                // For struct/class types, look up the byte_size from parsed types
                _ => {
                    // Look up the type size in our collected types
                    *self.type_sizes.get(&type_info.base_type).unwrap_or_else(|| {
                        panic!("Unknown type size for '{}'. Add it to the type size map or handle it in estimate_type_size.", type_info.base_type)
                    })
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
        let mut decl = var.type_info.to_string(&var.name);

        // Add bitfield specification if present
        if let Some(bit_size) = var.bit_size {
            decl.push_str(&format!(" : {}", bit_size));
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
                }
                Element::Namespace(ns) => {
                    Self::collect_type_sizes_from_elements(type_sizes, &ns.children);
                }
                _ => {}
            }
        }
    }

    pub fn generate_compile_unit(&mut self, cu: &CompileUnit) {
        self.write_line_comment("", &cu.name);
        if let Some(ref producer) = cu.producer {
            self.write_line(&format!("// Compiler: {}", producer));
        }
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

            // Add size comment
            if let Some(size) = compound.byte_size {
                line.push_str(&format!(" // sizeof: {}", size));
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

                        base_str.push_str(&base.type_name);

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

            // Add size comment
            if let Some(size) = compound.byte_size {
                closing.push_str(&format!(" // sizeof: {}", size));
            }

            self.write_line(&closing);
        }
    }

    fn generate_class(&mut self, compound: &Compound) {
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

                    base_str.push_str(&base.type_name);

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
            self.generate_members(private_members.to_vec().as_slice());
            for method in &private_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !protected_members.is_empty() || !protected_methods.is_empty() {
            self.indent_level += 1;
            self.write_line("protected:");
            self.generate_members(protected_members.to_vec().as_slice());
            for method in &protected_methods {
                self.generate_method(method);
            }
            self.indent_level -= 1;
        }

        if !public_members.is_empty() || !public_methods.is_empty() {
            self.indent_level += 1;
            self.write_line("public:");
            self.generate_members(public_members.to_vec().as_slice());
            for method in &public_methods {
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
        // Check if we have offset information for members - if so, use offset-based ordering
        let has_any_offsets = members.iter().any(|m| m.offset.is_some());

        if has_any_offsets {
            // Use offset-based ordering with padding detection
            let mut vars_with_offsets: Vec<_> = members
                .iter()
                .filter_map(|v| v.offset.map(|o| (o, *v)))
                .collect();
            vars_with_offsets.sort_by_key(|(offset, _)| *offset);

            let mut prev_end_offset = None;

            // Output members individually with offset information and padding
            for (offset, var) in vars_with_offsets {
                // Detect padding between members
                if let Some(prev_end) = prev_end_offset {
                    if offset > prev_end {
                        let padding_bytes = offset - prev_end;
                        self.write_line(&format!(
                            "// [{} byte{} padding for alignment]",
                            padding_bytes,
                            if padding_bytes == 1 { "" } else { "s" }
                        ));
                    }
                }

                let mut decl = self.format_member_declaration(var);
                decl.push(';');

                // Add line and offset comments
                if let Some(line) = var.line {
                    decl.push_str(&format!(" //{}", line));
                }
                decl.push_str(&format!(" @ offset {}", offset));

                // Add bit offset comment if it's a bitfield
                if let (Some(_bit_size), Some(bit_offset)) = (var.bit_size, var.bit_offset) {
                    decl.push_str(&format!(" [bit offset: {}]", bit_offset));
                }

                self.write_line(&decl);

                // Calculate end offset for next iteration
                let member_size = self.estimate_type_size(&var.type_info);
                prev_end_offset = Some(offset + member_size);
            }

            // Output members without offsets at the end
            for var in members.iter().filter(|v| v.offset.is_none()) {
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
        if !func.has_body
            || (func.variables.is_empty()
                && func.lexical_blocks.is_empty()
                && func.inlined_calls.is_empty()
                && func.labels.is_empty())
        {
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

        // Detect constructor: name matches class name
        let is_constructor = func.class_name.as_ref() == Some(&func.name);

        // Virtual keyword
        if func.is_virtual {
            decl.push_str("virtual ");
        }

        // Return type (skip for constructors/destructors)
        if !is_constructor && !func.is_destructor {
            decl.push_str(&func.return_type.base_type);
            if func.return_type.pointer_count > 0 {
                decl.push_str(&"*".repeat(func.return_type.pointer_count));
            }
            decl.push(' ');
        }

        // Function name
        decl.push_str(&func.name);
        decl.push('(');

        // Parameters (skip 'this' for methods)
        let params: Vec<_> = func
            .parameters
            .iter()
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

        // Add linkage name (demangle C++ names if possible, otherwise show raw)
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

        // Add artificial flag if it's compiler-generated
        if func.is_artificial {
            if !comment.is_empty() {
                comment.push(' ');
            }
            comment.push_str("[compiler-generated]");
        }

        comment
    }

    fn generate_function(&mut self, func: &Function) {
        // Write address comment above function if available
        if let (Some(low), Some(high)) = (func.low_pc, func.high_pc) {
            let size = high.saturating_sub(low);
            self.write_line(&format!("// @ 0x{:x}-0x{:x} ({} bytes)", low, high, size));
        }

        let decl = self.generate_function_declaration(func);

        if !func.has_body
            || (func.variables.is_empty()
                && func.lexical_blocks.is_empty()
                && func.inlined_calls.is_empty()
                && func.labels.is_empty())
        {
            // Function declaration only - add semicolon
            // The declaration may be multi-line with embedded line comments
            // Semicolon should be appended to the declaration string itself
            self.write_line(&format!("{};", decl));
        } else {
            self.write_line(&decl);

            // Check if we have a single lexical block at top level with no other variables
            let single_block = func.lexical_blocks.len() == 1
                && func.variables.is_empty()
                && func.inlined_calls.is_empty()
                && func.labels.is_empty();

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

        // Static/extern specifier (only for non-method functions)
        if !func.is_method && !func.is_external {
            decl.push_str("static ");
        }
        // Note: "extern" is implicit for external functions, so we don't output it

        // Inline specifier
        if func.is_inline {
            decl.push_str("inline ");
        }

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
        let params: Vec<_> = func
            .parameters
            .iter()
            .filter(|p| !(func.is_method && p.name == "this"))
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
                    decl.push_str(&param.type_info.to_string(&param.name));
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
                if let Some(line) = func.line {
                    decl.push_str(&format!(" //{}", line));
                }
                decl.push('\n');

                // Group parameters by line
                let mut param_lines: HashMap<Option<u64>, Vec<&Parameter>> = HashMap::new();
                for param in &params {
                    param_lines.entry(param.line).or_default().push(param);
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
            let line_comment = inlined
                .line
                .map(|l| format!(" //{}", l))
                .unwrap_or_default();
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
            let line_comment = inlined
                .line
                .map(|l| format!(" //{}", l))
                .unwrap_or_default();
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
        self.write_line(&format!(
            "{};{}",
            var.type_info.to_string(&var.name),
            line_comment
        ));
    }

    pub fn get_output(self) -> String {
        self.output
    }
}
