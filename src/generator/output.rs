//! Output writing utilities for code generation

/// Handles output buffering with indentation
pub struct OutputWriter {
    output: String,
    indent_level: usize,
}

impl OutputWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    /// Get the current indentation string
    fn indent_str(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    /// Increase indentation level
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease indentation level
    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Get current indent level
    #[allow(dead_code)]
    pub fn get_indent_level(&self) -> usize {
        self.indent_level
    }

    /// Set indent level directly
    #[allow(dead_code)]
    pub fn set_indent_level(&mut self, level: usize) {
        self.indent_level = level;
    }

    /// Write a line with current indentation
    pub fn write_line(&mut self, line: &str) {
        self.output.push_str(&self.indent_str());
        self.output.push_str(line);
        self.output.push('\n');
    }

    /// Write a line with a comment
    pub fn write_line_comment(&mut self, line: &str, comment: &str) {
        self.output.push_str(&self.indent_str());
        self.output.push_str(line);
        self.output.push_str(" //");
        self.output.push_str(comment);
        self.output.push('\n');
    }

    /// Push a newline
    pub fn push_newline(&mut self) {
        self.output.push('\n');
    }

    /// Push raw string (without indentation)
    pub fn push_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }

    /// Get the accumulated output
    pub fn get_output(self) -> String {
        self.output
    }

    /// Get a reference to the current output
    #[allow(dead_code)]
    pub fn output_ref(&self) -> &str {
        &self.output
    }
}

impl Default for OutputWriter {
    fn default() -> Self {
        Self::new()
    }
}
