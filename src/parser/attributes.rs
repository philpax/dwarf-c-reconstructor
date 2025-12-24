//! DWARF attribute extraction helpers

use crate::types::*;
use gimli::{AttributeValue, DebuggingInformationEntry, Dwarf, Reader};

/// Helper trait for extracting DWARF attributes from entries
pub struct AttributeExtractor<'a> {
    dwarf: &'a Dwarf<DwarfReader<'a>>,
}

impl<'a> AttributeExtractor<'a> {
    pub fn new(dwarf: &'a Dwarf<DwarfReader<'a>>) -> Self {
        Self { dwarf }
    }

    pub fn get_string_attr(
        &self,
        _unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<String> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::String(s) => {
                    if let Ok(slice) = s.to_slice() {
                        return Some(String::from_utf8_lossy(&slice).to_string());
                    }
                }
                AttributeValue::DebugStrRef(offset) => {
                    if let Ok(s) = self.dwarf.debug_str.get_str(offset) {
                        if let Ok(slice) = s.to_slice() {
                            return Some(String::from_utf8_lossy(&slice).to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_u64_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<u64> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::Udata(v) => return Some(v),
                AttributeValue::Data1(v) => return Some(v as u64),
                AttributeValue::Data2(v) => return Some(v as u64),
                AttributeValue::Data4(v) => return Some(v as u64),
                AttributeValue::Data8(v) => return Some(v),
                AttributeValue::Addr(v) => return Some(v),
                AttributeValue::FileIndex(v) => return Some(v),
                _ => {}
            }
        }
        None
    }

    pub fn get_i64_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<i64> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::Sdata(v) => return Some(v),
                AttributeValue::Udata(v) => return Some(v as i64),
                AttributeValue::Data1(v) => return Some(v as i64),
                AttributeValue::Data2(v) => return Some(v as i64),
                AttributeValue::Data4(v) => return Some(v as i64),
                AttributeValue::Data8(v) => return Some(v as i64),
                _ => {}
            }
        }
        None
    }

    pub fn get_bool_attr(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> bool {
        if let Some(attr_value) = entry.attr(attr).ok().flatten() {
            if let AttributeValue::Flag(v) = attr_value.value() {
                return v;
            }
        }
        false
    }

    pub fn get_member_offset(
        &self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<u64> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_data_member_location).ok()? {
            match attr_value.value() {
                AttributeValue::Udata(v) => return Some(v),
                AttributeValue::Data1(v) => return Some(v as u64),
                AttributeValue::Data2(v) => return Some(v as u64),
                AttributeValue::Data4(v) => return Some(v as u64),
                AttributeValue::Data8(v) => return Some(v),
                AttributeValue::Exprloc(expr) => {
                    // Try to evaluate simple DW_OP_plus_uconst expressions
                    let mut reader = expr.0;
                    let encoding = unit.header.encoding();
                    if let Ok(gimli::Operation::PlusConstant { value }) =
                        gimli::Operation::parse(&mut reader, encoding)
                    {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_ref_attr(
        &self,
        unit: &DwarfUnit,
        entry: &DebuggingInformationEntry<DwarfReader>,
        attr: gimli::DwAt,
    ) -> Option<usize> {
        if let Some(attr_value) = entry.attr(attr).ok()? {
            match attr_value.value() {
                AttributeValue::UnitRef(offset) => return Some(offset.0),
                AttributeValue::DebugInfoRef(offset) => {
                    // This is an absolute reference (DW_FORM_ref_addr)
                    // Convert to unit-relative offset if within this unit
                    let unit_base = unit
                        .header
                        .offset()
                        .as_debug_info_offset()
                        .map(|o| o.0)
                        .unwrap_or(0);
                    if offset.0 >= unit_base {
                        return Some(offset.0 - unit_base);
                    }
                    // If it's before our unit, it's a cross-unit reference
                    // which we can't resolve with unit-relative offsets
                    // Return None and let the caller handle it
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_accessibility(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<String> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_accessibility).ok()? {
            if let AttributeValue::Accessibility(access) = attr_value.value() {
                match access {
                    gimli::DwAccess(1) => return Some("public".to_string()),
                    gimli::DwAccess(2) => return Some("protected".to_string()),
                    gimli::DwAccess(3) => return Some("private".to_string()),
                    _ => {}
                }
            }
        }
        None
    }

    pub fn get_const_value(
        &self,
        entry: &DebuggingInformationEntry<DwarfReader>,
    ) -> Option<ConstValue> {
        if let Some(attr_value) = entry.attr(gimli::DW_AT_const_value).ok()? {
            match attr_value.value() {
                AttributeValue::Sdata(v) => return Some(ConstValue::Signed(v)),
                AttributeValue::Udata(v) => return Some(ConstValue::Unsigned(v)),
                AttributeValue::Data1(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data2(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data4(v) => return Some(ConstValue::Unsigned(v as u64)),
                AttributeValue::Data8(v) => return Some(ConstValue::Unsigned(v)),
                _ => {}
            }
        }
        None
    }
}
