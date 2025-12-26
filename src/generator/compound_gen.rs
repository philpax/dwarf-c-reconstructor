//! Compound type (struct/class/union/enum) code generation

use crate::types::*;
use std::collections::HashMap;

use super::function_gen::FunctionGenerator;
use super::output::OutputWriter;
use super::type_formatter::TypeFormatter;
use super::CodeGenConfig;

/// Generates code for compound types (struct, class, union, enum)
pub struct CompoundGenerator<'a> {
    output: &'a mut OutputWriter,
    formatter: TypeFormatter<'a>,
    config: &'a CodeGenConfig,
}

impl<'a> CompoundGenerator<'a> {
    pub fn new(
        output: &'a mut OutputWriter,
        config: &'a CodeGenConfig,
        type_sizes: &'a HashMap<String, u64>,
    ) -> Self {
        Self {
            output,
            formatter: TypeFormatter::new(config, type_sizes),
            config,
        }
    }

    /// Generate code for a compound type (dispatches to appropriate method)
    pub fn generate(&mut self, compound: &Compound, type_sizes: &HashMap<String, u64>) {
        // Check if we should merge typedef
        let use_typedef = compound.is_typedef && compound.typedef_name.is_some();

        if compound.compound_type == "enum" {
            self.generate_enum(compound, use_typedef);
        } else if compound.compound_type == "class" {
            self.generate_class(compound, type_sizes);
        } else {
            self.generate_struct_or_union(compound, use_typedef);
        }
    }

    /// Generate enum definition
    pub fn generate_enum(&mut self, compound: &Compound, use_typedef: bool) {
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

        self.output.write_line(&opening);

        // Enum values
        self.output.indent();
        for (name, value) in &compound.enum_values {
            if let Some(v) = value {
                self.output
                    .write_line(&format!("{} = {}, // 0x{:x}", name, v, v));
            } else {
                self.output.write_line(&format!("{},", name));
            }
        }
        self.output.dedent();

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

        self.output.write_line(&closing);
    }

    /// Generate struct or union definition
    pub fn generate_struct_or_union(&mut self, compound: &Compound, use_typedef: bool) {
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

            self.output.write_line(&line);
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
                    .map(|base| self.format_base_class(base))
                    .collect();
                opening.push_str(&bases.join(", "));
            }

            opening.push_str(" {");

            if let Some(line) = compound.line {
                opening.push_str(&format!(" //{}", line));
            } else if !self.config.disable_no_line_comment {
                opening.push_str(" //No line number");
            }

            self.output.write_line(&opening);

            self.output.indent();

            // Generate nested types first (they're typically declared at the top)
            for nested in &compound.nested_types {
                self.generate(nested, &HashMap::new());
                self.output.push_newline();
            }

            // Members grouped by line
            let member_refs: Vec<_> = compound.members.iter().collect();
            self.generate_members(&member_refs);
            self.output.dedent();

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

            self.output.write_line(&closing);
        }
    }

    /// Generate class definition
    pub fn generate_class(&mut self, compound: &Compound, type_sizes: &HashMap<String, u64>) {
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
            } else if !self.config.disable_no_line_comment {
                line.push_str(" //No line number");
            }

            self.output.write_line(&line);
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
                .map(|base| self.format_base_class(base))
                .collect();
            opening.push_str(&bases.join(", "));
        }

        opening.push_str(" {");

        if let Some(line) = compound.line {
            opening.push_str(&format!(" //{}", line));
        } else if !self.config.disable_no_line_comment {
            opening.push_str(" //No line number");
        }

        self.output.write_line(&opening);

        // Generate nested types first (they're typically declared at the top of the class)
        // Nested types default to private accessibility in classes
        if !compound.nested_types.is_empty() {
            self.output.indent();
            for nested in &compound.nested_types {
                self.generate(nested, type_sizes);
                self.output.push_newline();
            }
            self.output.dedent();
        }

        // Group members and methods by accessibility
        let mut public_members: Vec<&Variable> = Vec::new();
        let mut protected_members: Vec<&Variable> = Vec::new();
        let mut private_members: Vec<&Variable> = Vec::new();

        for member in &compound.members {
            match member.accessibility.as_deref() {
                Some("protected") => protected_members.push(member),
                Some("public") => public_members.push(member),
                Some("private") => private_members.push(member),
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
                None | Some(_) => public_methods.push(method),
            }
        }

        // Write sections - access specifiers at same indent level as class
        // Order: public, protected, private (conventional C++ style)
        if !public_members.is_empty() || !public_methods.is_empty() {
            self.output.write_line("public:");
            self.output.indent();
            self.generate_members(public_members.to_vec().as_slice());
            self.generate_methods(&public_methods, type_sizes);
            self.output.dedent();
        }

        if !protected_members.is_empty() || !protected_methods.is_empty() {
            self.output.write_line("protected:");
            self.output.indent();
            self.generate_members(protected_members.to_vec().as_slice());
            self.generate_methods(&protected_methods, type_sizes);
            self.output.dedent();
        }

        if !private_members.is_empty() || !private_methods.is_empty() {
            self.output.write_line("private:");
            self.output.indent();
            self.generate_members(private_members.to_vec().as_slice());
            self.generate_methods(&private_methods, type_sizes);
            self.output.dedent();
        }

        let mut closing = String::from("};");
        // Add size and vtable comments
        if let Some(size) = compound.byte_size {
            closing.push_str(&format!(" // sizeof: {}", size));
        }
        if compound.is_virtual {
            closing.push_str(" [has vtable]");
        }
        self.output.write_line(&closing);
    }

    /// Format a base class for inheritance declaration
    fn format_base_class(&self, base: &BaseClass) -> String {
        let mut base_str = String::new();

        // Add accessibility (defaults to public for structs, private for classes)
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
    }

    /// Generate methods sorted by line number
    fn generate_methods(&mut self, methods: &[&Function], type_sizes: &HashMap<String, u64>) {
        let mut sorted_methods: Vec<(usize, &Function)> =
            methods.iter().enumerate().map(|(i, &f)| (i, f)).collect();
        sorted_methods.sort_by_key(|(idx, m)| (m.line, *idx));

        for (_, method) in sorted_methods {
            let mut func_gen = FunctionGenerator::new(self.output, self.config, type_sizes);
            func_gen.generate_method(method);
        }
    }

    /// Generate member declarations
    pub fn generate_members(&mut self, members: &[&Variable]) {
        // Check if we have offset information for members
        let has_any_offsets = members.iter().any(|m| m.offset.is_some());

        if has_any_offsets {
            self.generate_members_with_offsets(members);
        } else {
            self.generate_members_without_offsets(members);
        }
    }

    /// Generate members with offset information
    fn generate_members_with_offsets(&mut self, members: &[&Variable]) {
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
            let last_member_size = self.formatter.estimate_type_size(&last_var.type_info);

            // Detect padding before this group
            if let Some(prev_end) = prev_end_offset {
                if first_offset > prev_end {
                    let padding_bytes = first_offset - prev_end;
                    self.output.write_line(&format!(
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
                    if self
                        .formatter
                        .types_compatible(&group[0].0.type_info, &var.type_info)
                    {
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
                        let mut decl = self.formatter.format_member_declaration(var);
                        if let (Some(_bit_size), Some(bit_offset)) = (var.bit_size, var.bit_offset)
                        {
                            decl.push_str(&format!(" [bit offset: {}]", bit_offset));
                        }
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

                    let transformed_base = self.formatter.transform_type_name(&base_type.base_type);
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
                self.output.write_line(&full_decl);
            }

            // Update prev_end_offset based on last member in this line group
            prev_end_offset = Some(last_offset + last_member_size);
        }

        // Output members without offsets at the end
        for var in no_offset_vars {
            let decl = self.formatter.format_member_declaration(var);
            if let Some(line) = var.line {
                self.output
                    .write_line_comment(&format!("{};", decl), &line.to_string());
            } else {
                self.output.write_line(&format!("{};", decl));
            }
        }
    }

    /// Generate members without offset information (line-based grouping)
    fn generate_members_without_offsets(&mut self, members: &[&Variable]) {
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
                    if self
                        .formatter
                        .types_compatible(&group[0].type_info, &var.type_info)
                    {
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
                        let decl = self.formatter.format_member_declaration(var);
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

                    let transformed_base = self.formatter.transform_type_name(&base_type.base_type);
                    let mut decl = transformed_base;
                    decl.push(' ');
                    decl.push_str(&var_names.join(", "));
                    decls.push(decl);
                }
            }

            let full_decl = decls.join("; ");
            self.output
                .write_line_comment(&format!("{};", full_decl), &line.to_string());
        }

        // Variables without line numbers
        for var in no_line_vars {
            let decl = self.formatter.format_member_declaration(var);
            self.output.write_line(&format!("{};", decl));
        }
    }
}
