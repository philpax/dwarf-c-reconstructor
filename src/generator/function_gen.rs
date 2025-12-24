//! Function and method code generation

use crate::types::*;
use cpp_demangle::Symbol;
use std::collections::{HashMap, VecDeque};

use super::output::OutputWriter;
use super::type_formatter::TypeFormatter;
use super::CodeGenConfig;

/// Generates code for functions and methods
pub struct FunctionGenerator<'a> {
    output: &'a mut OutputWriter,
    formatter: TypeFormatter<'a>,
    config: &'a CodeGenConfig,
}

impl<'a> FunctionGenerator<'a> {
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

    /// Generate a method declaration (inside a class)
    pub fn generate_method(&mut self, func: &Function) {
        // Inside a class definition, always output declaration-only format
        let decl = self.generate_method_declaration(func);
        self.output.write_line(&decl);
    }

    /// Generate method declaration string
    fn generate_method_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Detect constructor: name matches class name
        let is_constructor = func.class_name.as_ref() == Some(&func.name);

        // Virtual keyword
        if func.is_virtual {
            decl.push_str("virtual ");
        }

        // Return type (skip for constructors/destructors, apply type transformations)
        if !is_constructor && !func.is_destructor {
            decl.push_str(
                &self
                    .formatter
                    .transform_type_name(&func.return_type.base_type),
            );
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

        // Parameters - filter out implicit ones
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
            decl.push_str(
                &self
                    .formatter
                    .format_type_string(&param.type_info, &param.name),
            );
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

    /// Generate metadata comment for a function
    pub fn generate_function_metadata_comment(&self, func: &Function) -> String {
        let mut comment = String::new();

        // Add linkage name (demangle C++ names if possible) if not disabled
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

    /// Generate a top-level function
    pub fn generate_function(&mut self, func: &Function) {
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
            self.output.write_line(&format!("namespace {} {{", ns));
            if !self.config.skip_namespace_indentation {
                self.output.indent();
            } else {
                // Add empty line after opening bracket when skipping indentation
                self.output.push_newline();
            }
        }

        // Generate the function
        self.generate_function_impl(func);

        // Close namespace blocks in reverse order
        for ns in func.namespace_path.iter().rev() {
            if !self.config.skip_namespace_indentation {
                self.output.dedent();
            } else {
                // Add empty line before closing bracket when skipping indentation
                self.output.push_newline();
            }
            self.output.write_line(&format!("}} //{}", ns));
        }
    }

    /// Generate function implementation
    fn generate_function_impl(&mut self, func: &Function) {
        // Write address comment above function if available and not disabled
        if !self.config.no_function_addresses {
            if let (Some(low), Some(high)) = (func.low_pc, func.high_pc) {
                let size = high.saturating_sub(low);
                self.output
                    .write_line(&format!("// @ 0x{:x}-0x{:x} ({} bytes)", low, high, size));
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
            self.output.write_line(&decl_with_semicolon);
        } else {
            self.output.write_line(&decl);

            // Check if we have a single lexical block at top level with no other content
            let single_block_only = func.lexical_blocks.len() == 1
                && func.variables.is_empty()
                && func.inlined_calls.is_empty();

            if single_block_only {
                // Use function braces to contain the block's content
                self.output.write_line("{");
                self.output.indent();

                // Set up label queue for interleaving
                let mut pending_labels: VecDeque<&Label> = func.labels.iter().collect();
                pending_labels
                    .make_contiguous()
                    .sort_by_key(|l| (l.line, l.name.as_str()));

                // Output the lexical block's contents directly
                self.generate_lexical_block_contents_with_labels(
                    &func.lexical_blocks[0],
                    &mut pending_labels,
                );

                // Output any remaining labels at the end
                while let Some(label) = pending_labels.pop_front() {
                    let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
                    self.output
                        .push_raw(&format!("{}:{}\n", label.name, line_comment));
                }

                self.output.dedent();
                self.output.write_line("}");
            } else {
                // Add our own braces
                self.output.write_line("{");
                self.output.indent();
                self.generate_function_body(func);
                self.output.dedent();
                self.output.write_line("}");
            }
        }
    }

    /// Generate function declaration string
    fn generate_function_declaration(&self, func: &Function) -> String {
        let mut decl = String::new();

        // Static/extern specifier (only for non-method functions)
        if !func.is_method && !func.is_external {
            decl.push_str("static ");
        }

        // Inline specifier
        if func.is_inline {
            decl.push_str("inline ");
        }

        // Detect constructor: name matches class name
        let is_constructor = func.class_name.as_ref() == Some(&func.name);

        // Return type (skip for constructors/destructors)
        if !is_constructor && !func.is_destructor {
            decl.push_str(
                &self
                    .formatter
                    .transform_type_name(&func.return_type.base_type),
            );
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

        // Parameters - filter out implicit ones
        let params: Vec<_> = func
            .parameters
            .iter()
            .filter(|p| {
                if p.name == "this" && p.line.is_none() {
                    return false;
                }
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
                    decl.push_str(
                        &self
                            .formatter
                            .format_type_string(&param.type_info, &param.name),
                    );
                }
                decl.push(')');
                if let Some(line) = func.line {
                    decl.push_str(&format!(" //{}", line));
                }
                let metadata = self.generate_function_metadata_comment(func);
                if !metadata.is_empty() {
                    if func.line.is_none() {
                        decl.push_str(" //");
                    }
                    decl.push(' ');
                    decl.push_str(&metadata);
                }
            } else {
                // Parameters on different lines - group by line
                let mut param_lines: HashMap<Option<u64>, Vec<&Parameter>> = HashMap::new();
                for param in &params {
                    param_lines.entry(param.line).or_default().push(param);
                }

                let mut sorted_lines: Vec<_> = param_lines.iter().collect();
                sorted_lines.sort_by_key(|(line, _)| *line);

                for (idx, (line, params_at_line)) in sorted_lines.iter().enumerate() {
                    if idx == 0 {
                        // First params on same line as function name
                        for (i, param) in params_at_line.iter().enumerate() {
                            if i > 0 {
                                decl.push_str(", ");
                            }
                            decl.push_str(
                                &self
                                    .formatter
                                    .format_type_string(&param.type_info, &param.name),
                            );
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
                        // Subsequent groups on new lines with indentation
                        decl.push_str("        ");

                        for (i, param) in params_at_line.iter().enumerate() {
                            if i > 0 {
                                decl.push_str(", ");
                            }
                            decl.push_str(
                                &self
                                    .formatter
                                    .format_type_string(&param.type_info, &param.name),
                            );
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

    /// Insert a semicolon before the comment in a declaration
    fn insert_semicolon_before_comment(&self, decl: &str) -> String {
        if let Some(comment_pos) = decl.rfind(" //") {
            let mut result = String::new();
            result.push_str(&decl[..comment_pos]);
            result.push(';');
            result.push_str(&decl[comment_pos..]);
            result
        } else {
            format!("{};", decl)
        }
    }

    /// Generate function body
    fn generate_function_body(&mut self, func: &Function) {
        // Collect all function-level labels sorted by line number
        let mut pending_labels: VecDeque<&Label> = func.labels.iter().collect();
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
                    self.output_pending_labels_before(&mut pending_labels, var.line);
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
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
                    self.output_pending_labels_before(&mut pending_labels, inlined.line);
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
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
                    self.output_pending_labels_before(
                        &mut pending_labels,
                        block.min_content_line(),
                    );
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
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
                .push_raw(&format!("{}:{}\n", label.name, line_comment));
        }
    }

    /// Output pending labels that should appear before the given line
    fn output_pending_labels_before(
        &mut self,
        pending_labels: &mut VecDeque<&Label>,
        before_line: Option<u64>,
    ) {
        while let Some(label) = pending_labels.front() {
            match (label.line, before_line) {
                (Some(label_line), Some(elem_line)) if label_line < elem_line => {
                    let label = pending_labels.pop_front().unwrap();
                    self.output
                        .push_raw(&format!("{}: //{}\n", label.name, label_line));
                }
                (None, _) => {
                    let label = pending_labels.pop_front().unwrap();
                    self.output.push_raw(&format!("{}:\n", label.name));
                }
                _ => break,
            }
        }
    }

    /// Generate inlined calls grouped by line
    fn generate_inlined_calls(&mut self, inlined_calls: &[&InlinedSubroutine]) {
        if inlined_calls.is_empty() {
            return;
        }

        let mut line = String::new();

        for (i, inlined) in inlined_calls.iter().enumerate() {
            if i > 0 {
                line.push(' ');
            }
            line.push_str(&inlined.name);
            line.push_str("();");
        }

        // Add line comment from the first call
        if let Some(l) = inlined_calls[0].line {
            line.push_str(&format!(" //{}", l));
        }

        self.output.write_line(&line);
    }

    /// Generate local variables grouped by line
    fn generate_variables(&mut self, variables: &[&Variable]) {
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
                if group[0].type_info.is_function_pointer {
                    // Output function pointers individually
                    for var in group {
                        let decl = self.formatter.format_type_string(&var.type_info, &var.name);
                        decls.push(decl);
                    }
                } else {
                    let has_const_values = group.iter().any(|v| v.const_value.is_some());

                    if has_const_values {
                        // Output individually with const values
                        for var in group {
                            let mut decl =
                                self.formatter.format_type_string(&var.type_info, &var.name);
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

                        let transformed_base =
                            self.formatter.transform_type_name(&base_type.base_type);
                        let mut decl = transformed_base;
                        decl.push(' ');
                        decl.push_str(&var_names.join(", "));
                        decls.push(decl);
                    }
                }
            }

            let full_decl = decls.join("; ");
            self.output
                .write_line_comment(&format!("{};", full_decl), &line.to_string());
        }

        // Variables without line numbers
        for var in no_line_vars {
            self.output.write_line(&format!(
                "{};",
                self.formatter.format_type_string(&var.type_info, &var.name)
            ));
        }
    }

    /// Generate a lexical block with labels
    fn generate_lexical_block_with_labels(
        &mut self,
        block: &LexicalBlock,
        pending_labels: &mut VecDeque<&Label>,
    ) {
        self.output.write_line("{");
        self.output.indent();
        self.generate_lexical_block_contents_with_labels(block, pending_labels);
        self.output.dedent();
        self.output.write_line("}");
    }

    /// Generate the contents of a lexical block with label interleaving
    fn generate_lexical_block_contents_with_labels(
        &mut self,
        block: &LexicalBlock,
        pending_labels: &mut VecDeque<&Label>,
    ) {
        // Collect block-level labels
        let mut all_labels: VecDeque<&Label> = block.labels.iter().collect();
        all_labels
            .make_contiguous()
            .sort_by_key(|l| (l.line, l.name.as_str()));

        // Collect elements
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

        // Sort by line number
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

        // Generate in sorted order
        let mut variables_buffer: Vec<&Variable> = Vec::new();
        let mut inlined_buffer: Vec<&InlinedSubroutine> = Vec::new();
        let mut last_var_line: Option<u64> = None;
        let mut last_inlined_line: Option<u64> = None;

        for (_, element) in keyed_elements {
            match element {
                BlockElement::Variable(var, _) => {
                    self.output_pending_labels_before(pending_labels, var.line);
                    self.output_pending_labels_before(&mut all_labels, var.line);
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
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
                    self.output_pending_labels_before(pending_labels, inlined.line);
                    self.output_pending_labels_before(&mut all_labels, inlined.line);
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
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
                    self.output_pending_labels_before(pending_labels, nested.min_content_line());
                    self.output_pending_labels_before(&mut all_labels, nested.min_content_line());
                    if !variables_buffer.is_empty() {
                        self.generate_variables(&variables_buffer);
                        variables_buffer.clear();
                    }
                    if !inlined_buffer.is_empty() {
                        self.generate_inlined_calls(&inlined_buffer);
                        inlined_buffer.clear();
                    }
                    self.generate_lexical_block_with_labels(nested, pending_labels);
                }
            }
        }

        // Flush remaining
        if !variables_buffer.is_empty() {
            self.generate_variables(&variables_buffer);
        }
        if !inlined_buffer.is_empty() {
            self.generate_inlined_calls(&inlined_buffer);
        }

        // Output remaining block-level labels
        while let Some(label) = all_labels.pop_front() {
            let line_comment = label.line.map(|l| format!(" //{}", l)).unwrap_or_default();
            self.output
                .push_raw(&format!("{}:{}\n", label.name, line_comment));
        }
    }
}
