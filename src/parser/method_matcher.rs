//! Method definition matching logic
//!
//! Handles matching method declarations in classes with their definitions
//! at the top level, including cross-CU matching for archives.

use crate::types::*;
use std::collections::HashMap;

/// Match method declarations across all CUs using linkage names
/// This handles cases where a header is included in multiple CUs but the method
/// definitions are only in one CU (the .cpp file)
///
/// Note: This is public because it needs to be called after parsing all object files
/// in an archive, where CUs from different object files need to be matched together.
pub fn cross_cu_match_method_definitions(compile_units: &mut [CompileUnit]) {
    // First, collect all method definitions by linkage name from all CUs
    let mut definitions_by_linkage: HashMap<String, MethodDefinition> = HashMap::new();

    for cu in compile_units.iter() {
        collect_definitions(&cu.elements, &mut definitions_by_linkage);
    }

    // Then, match unmatched method declarations using the global map
    for cu in compile_units.iter_mut() {
        apply_matches(&mut cu.elements, &definitions_by_linkage);
    }
}

fn collect_definitions(elements: &[Element], definitions: &mut HashMap<String, MethodDefinition>) {
    for element in elements {
        match element {
            Element::Function(func) => {
                // Index any function that has parameters and a linkage name.
                // This handles both:
                // 1. Method definitions with specification_offset
                // 2. Functions that might match method declarations across CUs
                //    (where the definition may not have is_method set if the class
                //    declaration is in a different CU)
                if !func.parameters.is_empty() {
                    if let Some(ref linkage_name) = func.linkage_name {
                        definitions
                            .entry(linkage_name.clone())
                            .or_insert_with(|| MethodDefinition::from_function(func));
                    }
                }
            }
            Element::Namespace(ns) => {
                collect_definitions(&ns.children, definitions);
            }
            _ => {}
        }
    }
}

fn apply_matches(elements: &mut [Element], definitions: &HashMap<String, MethodDefinition>) {
    for element in elements.iter_mut() {
        match element {
            Element::Compound(compound) => {
                for method in &mut compound.methods {
                    // Only try cross-CU matching if method has no parameters yet
                    // (meaning intra-CU matching didn't find a definition)
                    if method.parameters.is_empty() {
                        if let Some(ref linkage_name) = method.linkage_name {
                            if let Some(def) = definitions.get(linkage_name) {
                                def.apply_to_method(method);
                            }
                        }
                    }
                }
            }
            Element::Namespace(ns) => {
                apply_matches(&mut ns.children, definitions);
            }
            _ => {}
        }
    }
}

/// Match method declarations in classes with their definitions at top level
pub fn match_method_definitions(elements: &mut [Element]) {
    match_method_definitions_with_namespace(elements, &[]);
}

/// Match method declarations with definitions, tracking namespace path
fn match_method_definitions_with_namespace(elements: &mut [Element], current_namespace: &[String]) {
    // Build maps of definitions:
    // 1. By linkage name (for methods that have it on both declaration and definition)
    // 2. By specification offset (for methods that use DW_AT_specification)
    let mut definitions_by_linkage: HashMap<String, MethodDefinition> = HashMap::new();
    let mut definitions_by_spec_offset: HashMap<usize, MethodDefinition> = HashMap::new();

    collect_method_definitions(
        elements,
        &mut definitions_by_linkage,
        &mut definitions_by_spec_offset,
    );

    // Match methods with definitions
    for element in elements.iter_mut() {
        match element {
            Element::Compound(compound) => {
                for method in &mut compound.methods {
                    // Set namespace_path for methods inside classes
                    if method.namespace_path.is_empty() && !current_namespace.is_empty() {
                        method.namespace_path = current_namespace.to_vec();
                    }

                    // First try to match by decl_offset (specification_offset on definition points to this)
                    let matched = if let Some(decl_offset) = method.decl_offset {
                        if let Some(def) = definitions_by_spec_offset.get(&decl_offset) {
                            def.apply_to_method(method);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    // Fall back to linkage name matching if specification matching didn't work
                    if !matched {
                        if let Some(ref linkage_name) = method.linkage_name {
                            if let Some(def) = definitions_by_linkage.get(linkage_name) {
                                def.apply_to_method(method);
                            }
                        }
                    }
                }
            }
            Element::Namespace(ns) => {
                let mut new_namespace = current_namespace.to_vec();
                new_namespace.push(ns.name.clone());
                match_method_definitions_with_namespace(&mut ns.children, &new_namespace);
            }
            _ => {}
        }
    }

    // Build mapping from decl_offset/linkage_name to (class_name, namespace_path) for marking top-level functions
    let mut class_info_by_decl_offset: HashMap<usize, (String, Vec<String>)> = HashMap::new();
    let mut class_info_by_linkage: HashMap<String, (String, Vec<String>)> = HashMap::new();

    collect_class_methods(
        elements,
        current_namespace,
        &mut class_info_by_decl_offset,
        &mut class_info_by_linkage,
    );

    // Mark top-level functions that are method definitions and set their class_name and namespace_path
    for element in elements.iter_mut() {
        if let Element::Function(func) = element {
            // If the function doesn't have a class_name yet, try to find it
            if func.class_name.is_none() {
                // Try to match by specification_offset first, then by linkage_name
                let class_info = func
                    .specification_offset
                    .and_then(|offset| class_info_by_decl_offset.get(&offset).cloned())
                    .or_else(|| {
                        func.linkage_name
                            .as_ref()
                            .and_then(|name| class_info_by_linkage.get(name).cloned())
                    });

                if let Some((class_name, namespace_path)) = class_info {
                    func.is_method = true;
                    func.class_name = Some(class_name);
                    if func.namespace_path.is_empty() {
                        func.namespace_path = namespace_path;
                    }
                }
            }
        }
    }
}

fn collect_method_definitions(
    elements: &[Element],
    definitions_by_linkage: &mut HashMap<String, MethodDefinition>,
    definitions_by_spec_offset: &mut HashMap<usize, MethodDefinition>,
) {
    for element in elements.iter() {
        match element {
            Element::Function(func) => {
                let definition_data = MethodDefinition::from_function(func);

                // If this function has a specification_offset, it's a definition referencing a declaration
                if let Some(spec_offset) = func.specification_offset {
                    definitions_by_spec_offset.insert(spec_offset, definition_data.clone());
                }

                // Also index by linkage name for backwards compatibility
                if let Some(ref linkage_name) = func.linkage_name {
                    if !func.is_method || func.specification_offset.is_some() {
                        definitions_by_linkage.insert(linkage_name.clone(), definition_data);
                    }
                }
            }
            Element::Namespace(ns) => {
                collect_method_definitions(
                    &ns.children,
                    definitions_by_linkage,
                    definitions_by_spec_offset,
                );
            }
            _ => {}
        }
    }
}

fn collect_class_methods(
    elements: &[Element],
    current_namespace: &[String],
    class_info_by_decl_offset: &mut HashMap<usize, (String, Vec<String>)>,
    class_info_by_linkage: &mut HashMap<String, (String, Vec<String>)>,
) {
    for element in elements.iter() {
        match element {
            Element::Compound(compound) => {
                if let Some(ref class_name) = compound.name {
                    for method in &compound.methods {
                        let ns_path = if !method.namespace_path.is_empty() {
                            method.namespace_path.clone()
                        } else {
                            current_namespace.to_vec()
                        };

                        if let Some(decl_offset) = method.decl_offset {
                            class_info_by_decl_offset
                                .insert(decl_offset, (class_name.clone(), ns_path.clone()));
                        }
                        if let Some(ref linkage_name) = method.linkage_name {
                            class_info_by_linkage.insert(
                                linkage_name.clone(),
                                (class_name.clone(), ns_path.clone()),
                            );
                        }
                    }
                }
            }
            Element::Namespace(ns) => {
                let mut new_namespace = current_namespace.to_vec();
                new_namespace.push(ns.name.clone());
                collect_class_methods(
                    &ns.children,
                    &new_namespace,
                    class_info_by_decl_offset,
                    class_info_by_linkage,
                );
            }
            _ => {}
        }
    }
}
