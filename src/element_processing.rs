//! Element processing utilities
//!
//! Provides functions for grouping, merging, and deduplicating elements
//! based on their declaration files.

use crate::types::{Compound, Element, Function, Namespace};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/// Configuration for element merging
#[derive(Debug, Clone, Default)]
pub struct MergeConfig {
    /// If true, show verbose details about what was merged
    pub verbose: bool,
    /// If true, skip merging anonymous types
    pub no_anonymous_merge: bool,
}

/// Statistics about merged anonymous types
#[derive(Debug, Clone, Default)]
pub struct MergeStats {
    /// Number of anonymous enums merged (count of duplicates removed)
    pub anonymous_enums_merged: usize,
    /// Number of anonymous structs merged
    pub anonymous_structs_merged: usize,
    /// Number of anonymous unions merged
    pub anonymous_unions_merged: usize,
    /// Detailed info about merged types (for verbose output)
    pub merge_details: Vec<MergeDetail>,
}

impl MergeStats {
    /// Returns total count of merged anonymous types
    pub fn total_merged(&self) -> usize {
        self.anonymous_enums_merged + self.anonymous_structs_merged + self.anonymous_unions_merged
    }

    /// Print summary to console
    pub fn print_summary(&self) {
        if self.anonymous_enums_merged > 0 {
            eprintln!(
                "Merged {} duplicate anonymous enum{}",
                self.anonymous_enums_merged,
                if self.anonymous_enums_merged == 1 {
                    ""
                } else {
                    "s"
                }
            );
        }
        if self.anonymous_structs_merged > 0 {
            eprintln!(
                "Merged {} duplicate anonymous struct{}",
                self.anonymous_structs_merged,
                if self.anonymous_structs_merged == 1 {
                    ""
                } else {
                    "s"
                }
            );
        }
        if self.anonymous_unions_merged > 0 {
            eprintln!(
                "Merged {} duplicate anonymous union{}",
                self.anonymous_unions_merged,
                if self.anonymous_unions_merged == 1 {
                    ""
                } else {
                    "s"
                }
            );
        }
    }

    /// Print verbose details about what was merged
    pub fn print_verbose_details(&self) {
        for detail in &self.merge_details {
            eprintln!("\nMerged the following {}s:", detail.compound_type);
            for repr in &detail.representations {
                eprintln!("{}", repr);
            }
        }
    }
}

/// Details about a specific merge operation
#[derive(Debug, Clone)]
pub struct MergeDetail {
    /// Type of compound (enum, struct, union)
    pub compound_type: String,
    /// Text representations of each merged instance
    pub representations: Vec<String>,
}

/// Generate a content-based key for anonymous compounds.
/// Anonymous types are matched by: line number + compound_type + content signature.
fn anonymous_compound_key(compound: &Compound) -> Option<String> {
    // Only create keys for anonymous types (no name and no typedef_name)
    if compound.name.is_some() || compound.typedef_name.is_some() {
        return None;
    }

    let mut key = format!("anon:{}:", compound.compound_type);

    // Add line number if available
    if let Some(line) = compound.line {
        key.push_str(&format!("L{}:", line));
    } else {
        key.push_str("L?:");
    }

    // Add content signature based on compound type
    if compound.compound_type == "enum" {
        // For enums: use enum values with their names and values
        let mut values_sig = String::new();
        for (name, value) in &compound.enum_values {
            values_sig.push_str(name);
            if let Some(v) = value {
                values_sig.push_str(&format!("={}", v));
            }
            values_sig.push(',');
        }
        key.push_str(&values_sig);
    } else {
        // For structs/unions: use member names, types, and line numbers
        let mut members_sig = String::new();
        for member in &compound.members {
            members_sig.push_str(&member.name);
            members_sig.push(':');
            members_sig.push_str(&member.type_info.base_type);
            if let Some(line) = member.line {
                members_sig.push_str(&format!("@{}", line));
            }
            members_sig.push(',');
        }
        key.push_str(&members_sig);
    }

    // Add byte_size if available for additional matching
    if let Some(size) = compound.byte_size {
        key.push_str(&format!(":sz{}", size));
    }

    Some(key)
}

/// Format a compound type for verbose output
fn format_compound_for_display(compound: &Compound) -> String {
    let mut output = String::new();

    // Opening line
    output.push_str(&compound.compound_type);
    output.push_str(" {");
    if let Some(line) = compound.line {
        output.push_str(&format!(" //{}", line));
    }
    output.push('\n');

    // Content
    if compound.compound_type == "enum" {
        for (name, value) in &compound.enum_values {
            if let Some(v) = value {
                output.push_str(&format!("    {} = {}, // 0x{:x}\n", name, v, v));
            } else {
                output.push_str(&format!("    {},\n", name));
            }
        }
    } else {
        for member in &compound.members {
            output.push_str(&format!(
                "    {} {};\n",
                member.type_info.base_type, member.name
            ));
        }
    }

    // Closing line
    output.push_str("};");
    if let Some(size) = compound.byte_size {
        output.push_str(&format!(" // sizeof: {}", size));
    }

    output
}

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

/// Merge namespaces with configuration and return statistics.
/// This version allows controlling anonymous type merging and collecting stats.
pub fn merge_namespaces_with_config(
    elements: Vec<Element>,
    config: &MergeConfig,
) -> (Vec<Element>, MergeStats) {
    let mut stats = MergeStats::default();

    // Separate namespaces from other elements
    let mut namespaces_by_name: BTreeMap<String, (Namespace, HashSet<String>, HashSet<String>)> =
        BTreeMap::new();
    let mut other_elements: Vec<Element> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();
    // Track anonymous types separately for merging
    let mut anonymous_types: HashMap<String, Vec<Compound>> = HashMap::new();

    for element in elements {
        match element {
            Element::Namespace(ns) => {
                // If we already have a namespace with this name, merge the children
                if let Some((existing, existing_keys, existing_anon_keys)) =
                    namespaces_by_name.get_mut(&ns.name)
                {
                    // Add new children, deduplicating against existing
                    for child in ns.children {
                        if let Element::Namespace(_) = &child {
                            // Namespaces will be merged recursively, always add them
                            existing.children.push(child);
                        } else if let Element::Compound(ref c) = child {
                            // Check for anonymous compound
                            if !config.no_anonymous_merge {
                                if let Some(anon_key) = anonymous_compound_key(c) {
                                    if !existing_anon_keys.insert(anon_key) {
                                        // Duplicate anonymous type - skip it but track for stats
                                        match c.compound_type.as_str() {
                                            "enum" => stats.anonymous_enums_merged += 1,
                                            "struct" => stats.anonymous_structs_merged += 1,
                                            "union" => stats.anonymous_unions_merged += 1,
                                            _ => {}
                                        }
                                        continue;
                                    }
                                }
                            }
                            // Try named key deduplication
                            if let Some(key) = element_key(&child) {
                                if existing_keys.insert(key) {
                                    existing.children.push(child);
                                }
                            } else {
                                existing.children.push(child);
                            }
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
                    let mut anon_keys: HashSet<String> = HashSet::new();
                    for child in &ns.children {
                        if let Some(key) = element_key(child) {
                            child_keys.insert(key);
                        }
                        if !config.no_anonymous_merge {
                            if let Element::Compound(c) = child {
                                if let Some(anon_key) = anonymous_compound_key(c) {
                                    anon_keys.insert(anon_key);
                                }
                            }
                        }
                    }
                    namespaces_by_name.insert(ns.name.clone(), (ns, child_keys, anon_keys));
                }
            }
            Element::Compound(ref c) => {
                // Check for anonymous compound at top level
                if !config.no_anonymous_merge {
                    if let Some(anon_key) = anonymous_compound_key(c) {
                        // Track for potential merging
                        anonymous_types
                            .entry(anon_key.clone())
                            .or_default()
                            .push(c.clone());

                        // Check if we've seen this key before
                        if !seen_keys.insert(anon_key) {
                            // Duplicate - skip but track for stats
                            match c.compound_type.as_str() {
                                "enum" => stats.anonymous_enums_merged += 1,
                                "struct" => stats.anonymous_structs_merged += 1,
                                "union" => stats.anonymous_unions_merged += 1,
                                _ => {}
                            }
                            continue;
                        }
                        other_elements.push(element);
                        continue;
                    }
                }
                // Named compound - use regular deduplication
                if let Some(key) = element_key(&Element::Compound(c.clone())) {
                    if seen_keys.insert(key) {
                        other_elements.push(element);
                    }
                } else {
                    other_elements.push(element);
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

    // Build verbose merge details if enabled
    if config.verbose {
        for compounds in anonymous_types.values() {
            if compounds.len() > 1 {
                let compound_type = compounds[0].compound_type.clone();
                let representations: Vec<String> =
                    compounds.iter().map(format_compound_for_display).collect();
                stats.merge_details.push(MergeDetail {
                    compound_type,
                    representations,
                });
            }
        }
    }

    // Recursively merge nested namespaces in each namespace's children
    let mut result: Vec<Element> = namespaces_by_name
        .into_values()
        .map(|(mut ns, _, _)| {
            let (merged_children, child_stats) = merge_namespaces_with_config(ns.children, config);
            ns.children = merged_children;
            // Accumulate stats from nested namespaces
            stats.anonymous_enums_merged += child_stats.anonymous_enums_merged;
            stats.anonymous_structs_merged += child_stats.anonymous_structs_merged;
            stats.anonymous_unions_merged += child_stats.anonymous_unions_merged;
            stats.merge_details.extend(child_stats.merge_details);
            Element::Namespace(ns)
        })
        .collect();
    result.extend(other_elements);

    (result, stats)
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

/// Wrap method definitions with namespace paths into proper Element::Namespace structures.
/// This allows merge_namespaces() to properly merge methods from the same namespace.
pub fn wrap_method_definitions_in_namespaces(elements: Vec<Element>) -> Vec<Element> {
    wrap_method_definitions_with_context(elements, &[])
}

/// Recursively wrap method definitions, tracking the current namespace context.
/// Functions inside a namespace element should have their namespace_path prefix stripped
/// to avoid creating redundant nested namespaces.
fn wrap_method_definitions_with_context(
    elements: Vec<Element>,
    current_namespace: &[String],
) -> Vec<Element> {
    let mut result = Vec::new();

    for element in elements {
        match element {
            Element::Function(mut func) if func.is_method && !func.namespace_path.is_empty() => {
                // Check if the function's namespace_path starts with the current namespace context
                let remaining_path = if func.namespace_path.starts_with(current_namespace) {
                    func.namespace_path[current_namespace.len()..].to_vec()
                } else {
                    // Namespace path doesn't match context - keep as is (shouldn't happen normally)
                    func.namespace_path.clone()
                };

                if remaining_path.is_empty() {
                    // Function is already in the correct namespace, just clear its namespace_path
                    func.namespace_path.clear();
                    result.push(Element::Function(func));
                } else {
                    // Wrap in the remaining namespace levels
                    func.namespace_path.clear();
                    let wrapped = wrap_function_in_namespace_path(func, remaining_path);
                    result.push(wrapped);
                }
            }
            Element::Namespace(mut ns) => {
                // Build the new context by appending this namespace's name
                let mut new_context = current_namespace.to_vec();
                new_context.push(ns.name.clone());

                // Recursively process children with the updated context
                ns.children = wrap_method_definitions_with_context(ns.children, &new_context);
                result.push(Element::Namespace(ns));
            }
            _ => {
                // Keep other elements as-is
                result.push(element);
            }
        }
    }

    result
}

/// Wrap a function in nested Namespace elements based on a namespace path.
fn wrap_function_in_namespace_path(func: Function, namespace_path: Vec<String>) -> Element {
    if namespace_path.is_empty() {
        return Element::Function(func);
    }

    // Build nested namespaces from innermost to outermost
    let mut current = Element::Function(func);
    let line = match &current {
        Element::Function(f) => f.line,
        _ => None,
    };

    for ns_name in namespace_path.into_iter().rev() {
        current = Element::Namespace(Namespace {
            name: ns_name,
            line,
            children: vec![current],
        });
    }

    current
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
