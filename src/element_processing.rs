//! Element processing utilities
//!
//! Provides functions for grouping, merging, and deduplicating elements
//! based on their declaration files.

use crate::types::{Element, Namespace};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/// Get the decl_file for an element
pub fn get_element_decl_file(element: &Element) -> Option<u64> {
    match element {
        Element::Compound(c) => c.decl_file,
        Element::Function(f) => f.decl_file,
        Element::Variable(v) => v.decl_file,
        Element::Namespace(_) => None, // Namespaces themselves don't have decl_file
        Element::TypedefAlias(t) => t.decl_file,
    }
}

/// Information about a namespace element split by decl_file
pub struct NamespaceByFile {
    /// The original namespace name
    pub name: String,
    /// The original namespace line
    pub line: Option<u64>,
    /// Children grouped by their decl_file
    pub children_by_file: HashMap<Option<u64>, Vec<Element>>,
}

/// Split namespace children by their decl_file values
pub fn split_namespace_by_file(ns: &Namespace) -> NamespaceByFile {
    let mut children_by_file: HashMap<Option<u64>, Vec<Element>> = HashMap::new();

    for child in &ns.children {
        let decl_file = get_element_decl_file(child);

        // For nested namespaces, recursively split them
        if let Element::Namespace(nested_ns) = child {
            let nested_split = split_namespace_by_file(nested_ns);
            for (file, nested_children) in nested_split.children_by_file {
                if !nested_children.is_empty() {
                    // Create a new namespace containing just the children for this file
                    let filtered_ns = Namespace {
                        name: nested_split.name.clone(),
                        line: nested_split.line,
                        children: nested_children,
                    };
                    children_by_file
                        .entry(file)
                        .or_default()
                        .push(Element::Namespace(filtered_ns));
                }
            }
        } else {
            // Clone the element for each file it belongs to
            children_by_file
                .entry(decl_file)
                .or_default()
                .push(child.clone());
        }
    }

    NamespaceByFile {
        name: ns.name.clone(),
        line: ns.line,
        children_by_file,
    }
}

/// Generate a unique key for an element, used for deduplication.
pub fn element_key(element: &Element) -> Option<String> {
    match element {
        Element::Compound(c) => {
            // Key based on name and compound type
            // For typedef compounds (e.g., typedef struct { } Foo;), use typedef_name
            let name = c.name.as_ref().or(c.typedef_name.as_ref());
            name.map(|n| format!("{}:{}", c.compound_type, n))
        }
        Element::TypedefAlias(t) => Some(format!("typedef:{}", t.name)),
        Element::Function(f) => {
            // Key based on linkage name if available, otherwise name
            Some(format!(
                "func:{}",
                f.linkage_name.as_ref().unwrap_or(&f.name)
            ))
        }
        Element::Variable(v) => Some(format!("var:{}", v.name)),
        Element::Namespace(_) => None, // Namespaces are handled separately by merging
    }
}

/// Merge namespaces with the same name and deduplicate other elements.
/// This prevents duplicate definitions in the output when elements from
/// multiple compile units are merged together.
pub fn merge_namespaces(elements: Vec<Element>) -> Vec<Element> {
    // Separate namespaces from other elements
    let mut namespaces_by_name: BTreeMap<String, (Namespace, HashSet<String>)> = BTreeMap::new();
    let mut other_elements: Vec<Element> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();

    for element in elements {
        match element {
            Element::Namespace(ns) => {
                // If we already have a namespace with this name, merge the children
                if let Some((existing, existing_keys)) = namespaces_by_name.get_mut(&ns.name) {
                    // Add new children, deduplicating against existing
                    for child in ns.children {
                        if let Element::Namespace(_) = &child {
                            // Namespaces will be merged recursively, always add them
                            existing.children.push(child);
                        } else if let Some(key) = element_key(&child) {
                            if existing_keys.insert(key) {
                                existing.children.push(child);
                            }
                            // Skip duplicates
                        } else {
                            existing.children.push(child);
                        }
                    }
                    // Use the earlier line number
                    if ns.line < existing.line {
                        existing.line = ns.line;
                    }
                } else {
                    // First time seeing this namespace - collect keys from children
                    let mut child_keys: HashSet<String> = HashSet::new();
                    for child in &ns.children {
                        if let Some(key) = element_key(child) {
                            child_keys.insert(key);
                        }
                    }
                    namespaces_by_name.insert(ns.name.clone(), (ns, child_keys));
                }
            }
            _ => {
                // Deduplicate non-namespace elements
                if let Some(key) = element_key(&element) {
                    if seen_keys.insert(key) {
                        // First time seeing this element
                        other_elements.push(element);
                    }
                    // Skip duplicates
                } else {
                    // No key (shouldn't happen for non-namespace elements)
                    other_elements.push(element);
                }
            }
        }
    }

    // Recursively merge nested namespaces in each namespace's children
    let mut result: Vec<Element> = namespaces_by_name
        .into_values()
        .map(|(mut ns, _)| {
            ns.children = merge_namespaces(ns.children);
            Element::Namespace(ns)
        })
        .collect();
    result.extend(other_elements);

    result
}

/// Group elements by their declaration file, properly handling namespaces
/// by splitting their children by decl_file and wrapping them in namespace elements
pub fn group_elements_by_file(elements: &[Element]) -> HashMap<Option<u64>, Vec<Element>> {
    let mut elements_by_file: HashMap<Option<u64>, Vec<Element>> = HashMap::new();

    for element in elements {
        match element {
            Element::Namespace(ns) => {
                // Split namespace children by their decl_file
                let split = split_namespace_by_file(ns);

                for (file, children) in split.children_by_file {
                    if !children.is_empty() {
                        // Create a new namespace containing just the children for this file
                        let filtered_ns = Namespace {
                            name: split.name.clone(),
                            line: split.line,
                            children,
                        };
                        elements_by_file
                            .entry(file)
                            .or_default()
                            .push(Element::Namespace(filtered_ns));
                    }
                }
            }
            _ => {
                let decl_file = get_element_decl_file(element);
                elements_by_file
                    .entry(decl_file)
                    .or_default()
                    .push(element.clone());
            }
        }
    }

    elements_by_file
}

/// Normalize a file path by removing .. and . components
pub fn normalize_path(path: &str) -> String {
    if path.is_empty() {
        return "unknown.c".to_string();
    }

    let path_obj = Path::new(path);
    let mut components = Vec::new();

    for component in path_obj.components() {
        match component {
            std::path::Component::Normal(c) => {
                components.push(c.to_str().unwrap_or("unknown"));
            }
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {
                // Skip
            }
            _ => {}
        }
    }

    if components.is_empty() {
        "unknown.c".to_string()
    } else {
        components.join("/")
    }
}
