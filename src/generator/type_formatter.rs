//! Type formatting and transformation utilities

use crate::types::*;
use std::collections::HashMap;

use super::CodeGenConfig;

/// Handles type formatting and transformation
pub struct TypeFormatter<'a> {
    config: &'a CodeGenConfig,
    type_sizes: &'a HashMap<String, u64>,
}

impl<'a> TypeFormatter<'a> {
    pub fn new(config: &'a CodeGenConfig, type_sizes: &'a HashMap<String, u64>) -> Self {
        Self { config, type_sizes }
    }

    /// Shorten integer type names (e.g., "short int" -> "short")
    pub fn shorten_type_name(&self, type_name: &str) -> String {
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
    pub fn strip_compound_prefix(&self, type_name: &str) -> String {
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
    pub fn transform_type_name(&self, type_name: &str) -> String {
        let mut result = type_name.to_string();

        // First strip compound prefixes (unless verbose_class_usage is enabled)
        result = self.strip_compound_prefix(&result);

        // Then apply int type shortening if enabled
        if self.config.shorten_int_types {
            result = self.shorten_type_name(&result);
        }

        result
    }

    /// Format a type with a variable name
    pub fn format_type_string(&self, type_info: &TypeInfo, var_name: &str) -> String {
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

    /// Estimate the size of a type in bytes
    pub fn estimate_type_size(&self, type_info: &TypeInfo) -> u64 {
        let ptr_size = self.config.pointer_size;

        // Determine the base element size
        let base_size = if type_info.pointer_count > 0 || type_info.is_function_pointer {
            // Pointers use architecture-specific size
            ptr_size
        } else {
            // Calculate size based on base type
            self.get_base_type_size(&type_info.base_type, ptr_size)
        };

        // If there are arrays, multiply by total array size
        // This handles both arrays of base types and arrays of pointers
        if !type_info.array_sizes.is_empty() {
            let total_elements: usize = type_info.array_sizes.iter().product();
            return base_size * (total_elements as u64);
        }

        base_size
    }

    /// Get the size of a base type
    fn get_base_type_size(&self, base_type: &str, ptr_size: u64) -> u64 {
        match base_type {
            "char" | "unsigned char" | "signed char" | "bool" | "boolean" => 1,
            "short" | "short int" | "unsigned short" | "signed short" | "short unsigned int" => 2,
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
            _ => *self.type_sizes.get(base_type).unwrap_or(&4), // Default to 4 bytes if unknown
        }
    }

    /// Format a member declaration with optional bitfield and const value
    pub fn format_member_declaration(&self, var: &Variable) -> String {
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

    /// Check if two types are compatible for grouping on the same line
    pub fn types_compatible(&self, t1: &TypeInfo, t2: &TypeInfo) -> bool {
        // Two types are compatible for joining if they have the same base type
        // and differ only in pointer count or array sizes
        t1.base_type == t2.base_type && !t1.is_function_pointer && !t2.is_function_pointer
    }
}
