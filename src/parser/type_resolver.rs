//! Type resolution from DWARF type references

use crate::error::Result;
use crate::types::*;
use gimli::DebuggingInformationEntry;
use std::collections::HashMap;

use super::attributes::AttributeExtractor;

/// Handles type resolution from DWARF type references
pub struct TypeResolver<'a> {
    attrs: AttributeExtractor<'a>,
    type_cache: &'a mut HashMap<usize, TypeInfo>,
    typedef_map: &'a HashMap<usize, TypedefInfo>,
}

impl<'a> TypeResolver<'a> {
    pub fn new(
        attrs: AttributeExtractor<'a>,
        type_cache: &'a mut HashMap<usize, TypeInfo>,
        typedef_map: &'a HashMap<usize, TypedefInfo>,
    ) -> Self {
        Self {
            attrs,
            type_cache,
            typedef_map,
        }
    }

    pub fn resolve_type(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        let type_offset = match self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
            Some(offset) => offset,
            None => return Ok(TypeInfo::new("void".to_string())),
        };

        self.resolve_type_from_offset(unit, type_offset)
    }

    pub fn resolve_type_from_offset(
        &mut self,
        unit: &DwarfUnit,
        offset: usize,
    ) -> Result<TypeInfo> {
        // Convert unit-relative offset to absolute offset for caching
        let unit_start = unit.header.offset().as_debug_info_offset().unwrap().0;
        let absolute_offset = unit_start + offset;

        // Check cache using absolute offset
        if let Some(cached) = self.type_cache.get(&absolute_offset) {
            return Ok(cached.clone());
        }

        let unit_offset = gimli::UnitOffset(offset);
        let mut entries = unit.entries_at_offset(unit_offset)?;

        if let Some((_, type_entry)) = entries.next_dfs()? {
            let type_info = self.resolve_type_entry(unit, type_entry)?;
            // Cache using absolute offset
            self.type_cache.insert(absolute_offset, type_info.clone());
            return Ok(type_info);
        }

        Ok(TypeInfo::new("void".to_string()))
    }

    pub fn resolve_type_entry(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        self.resolve_type_entry_impl(unit, entry, true)
    }

    /// Resolve a type entry without typedef substitution for compound types.
    /// Used when we need the raw underlying type (e.g., for generating typedef aliases).
    pub fn resolve_type_entry_raw(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Result<TypeInfo> {
        self.resolve_type_entry_impl(unit, entry, false)
    }

    fn resolve_type_entry_impl(
        &mut self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        use_typedef_substitution: bool,
    ) -> Result<TypeInfo> {
        match entry.tag() {
            gimli::DW_TAG_base_type => {
                let name = self
                    .attrs
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "void".to_string());
                Ok(TypeInfo::new(name))
            }
            gimli::DW_TAG_pointer_type => {
                if let Some(pointed_offset) =
                    self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type)
                {
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
                let base_offset = self
                    .attrs
                    .get_ref_attr(unit, entry, gimli::DW_AT_type)
                    .unwrap_or(0);
                let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;

                // Get array dimensions from subrange children
                let mut entries = unit.entries_at_offset(entry.offset())?;
                entries.next_dfs()?; // Skip the array type itself
                let mut absolute_depth = 0;

                while let Some((depth_delta, child_entry)) = entries.next_dfs()? {
                    absolute_depth += depth_delta;
                    if absolute_depth <= 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_subrange_type {
                        let size = if let Some(count) =
                            self.attrs.get_u64_attr(child_entry, gimli::DW_AT_count)
                        {
                            count as usize
                        } else if let Some(upper) = self
                            .attrs
                            .get_u64_attr(child_entry, gimli::DW_AT_upper_bound)
                        {
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
                if let Some(base_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_const = true;
                    Ok(type_info)
                } else {
                    Ok(TypeInfo::new("const void".to_string()))
                }
            }
            gimli::DW_TAG_volatile_type => {
                if let Some(base_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_volatile = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_volatile = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_restrict_type => {
                if let Some(base_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, base_offset)?;
                    type_info.is_restrict = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_restrict = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_reference_type => {
                if let Some(ref_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, ref_offset)?;
                    type_info.is_reference = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_reference = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_rvalue_reference_type => {
                if let Some(ref_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    let mut type_info = self.resolve_type_from_offset(unit, ref_offset)?;
                    type_info.is_rvalue_reference = true;
                    Ok(type_info)
                } else {
                    let mut type_info = TypeInfo::new("void".to_string());
                    type_info.is_rvalue_reference = true;
                    Ok(type_info)
                }
            }
            gimli::DW_TAG_typedef => {
                let name = self
                    .attrs
                    .get_string_attr(unit, entry, gimli::DW_AT_name)
                    .unwrap_or_else(|| "void".to_string());
                Ok(TypeInfo::new(name))
            }
            gimli::DW_TAG_structure_type
            | gimli::DW_TAG_class_type
            | gimli::DW_TAG_union_type
            | gimli::DW_TAG_enumeration_type => {
                let name = self.attrs.get_string_attr(unit, entry, gimli::DW_AT_name);

                if let Some(n) = name {
                    let prefix = match entry.tag() {
                        gimli::DW_TAG_structure_type => "struct ",
                        gimli::DW_TAG_class_type => "class ",
                        gimli::DW_TAG_union_type => "union ",
                        gimli::DW_TAG_enumeration_type => "enum ",
                        _ => "",
                    };

                    // Check if it has a typedef (only if substitution is enabled)
                    if use_typedef_substitution {
                        let offset = entry.offset().0;
                        if let Some(typedef_info) = self.typedef_map.get(&offset) {
                            return Ok(TypeInfo::new(typedef_info.name.clone()));
                        }
                    }
                    Ok(TypeInfo::new(format!("{}{}", prefix, n)))
                } else {
                    // Anonymous type, check for typedef (only if substitution is enabled)
                    if use_typedef_substitution {
                        let offset = entry.offset().0;
                        if let Some(typedef_info) = self.typedef_map.get(&offset) {
                            return Ok(TypeInfo::new(typedef_info.name.clone()));
                        }
                    }
                    Ok(TypeInfo::new("void".to_string()))
                }
            }
            gimli::DW_TAG_subroutine_type => {
                // Function pointer
                let mut func_type = TypeInfo::new("void".to_string());
                func_type.is_function_pointer = true;

                // Get return type
                if let Some(ret_offset) = self.attrs.get_ref_attr(unit, entry, gimli::DW_AT_type) {
                    if let Ok(ret_type) = self.resolve_type_from_offset(unit, ret_offset) {
                        func_type.function_return_type = Some(Box::new(ret_type));
                    }
                }

                // Get parameters
                let mut entries = unit.entries_at_offset(entry.offset())?;
                entries.next_dfs()?; // Skip the subroutine type itself
                let mut absolute_depth = 0;

                while let Some((depth_delta, child_entry)) = entries.next_dfs()? {
                    absolute_depth += depth_delta;
                    if absolute_depth <= 0 {
                        break;
                    }
                    if child_entry.tag() == gimli::DW_TAG_formal_parameter {
                        // Try to get the parameter type
                        if let Some(param_offset) =
                            self.attrs
                                .get_ref_attr(unit, child_entry, gimli::DW_AT_type)
                        {
                            if let Ok(param_type) =
                                self.resolve_type_from_offset(unit, param_offset)
                            {
                                func_type.function_params.push(param_type);
                            } else {
                                // Cross-unit or unresolvable reference - use void* as fallback
                                let mut void_ptr = TypeInfo::new("void".to_string());
                                void_ptr.pointer_count = 1;
                                func_type.function_params.push(void_ptr);
                            }
                        } else {
                            // No type attribute or unresolvable - check if there's a name
                            // For unnamed parameters with no type, assume void* if formal_parameter exists
                            let mut void_ptr = TypeInfo::new("void".to_string());
                            void_ptr.pointer_count = 1;
                            func_type.function_params.push(void_ptr);
                        }
                    }
                }

                Ok(func_type)
            }
            _ => Ok(TypeInfo::new("void".to_string())),
        }
    }
}
