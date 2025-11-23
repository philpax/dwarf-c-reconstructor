mod generator;
mod parser;
mod types;

use clap::Parser as ClapParser;
use generator::{CodeGenConfig, CodeGenerator};
use parser::DwarfParser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// DWARF C reconstructor - generates C++ code from DWARF debugging information
#[derive(ClapParser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(value_name = "ELF_FILE")]
    file_path: String,

    /// Output directory for generated files
    #[arg(short, long, default_value = "output")]
    output_dir: String,

    /// Shorten integer type names (e.g., "short int" becomes "short")
    #[arg(long)]
    shorten_int_types: bool,

    /// Remove function address comments
    #[arg(long)]
    no_function_addresses: bool,

    /// Remove all offset comments for struct members
    #[arg(long)]
    no_offsets: bool,

    /// Remove function prototype comments
    #[arg(long)]
    no_function_prototypes: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read file into static memory
    let file_data = fs::read(&args.file_path)?;
    let static_data: &'static [u8] = Box::leak(file_data.into_boxed_slice());

    // Parse DWARF
    let mut parser = DwarfParser::new(static_data)?;
    let compile_units = parser.parse()?;

    // Collect all type sizes from all compile units
    let mut type_sizes = HashMap::new();
    for cu in &compile_units {
        CodeGenerator::collect_type_sizes_from_elements(&mut type_sizes, &cu.elements);
    }

    // Generate code for each compile unit
    let output_dir = Path::new(&args.output_dir);
    fs::create_dir_all(output_dir)?;

    // Create config from command-line args
    let config = CodeGenConfig {
        shorten_int_types: args.shorten_int_types,
        no_function_addresses: args.no_function_addresses,
        no_offsets: args.no_offsets,
        no_function_prototypes: args.no_function_prototypes,
    };

    for cu in &compile_units {
        // Group elements by declaration file
        let mut elements_by_file: HashMap<Option<u64>, Vec<&types::Element>> = HashMap::new();

        for element in &cu.elements {
            let decl_file = match element {
                types::Element::Compound(c) => c.decl_file,
                types::Element::Function(f) => f.decl_file,
                types::Element::Variable(v) => v.decl_file,
                types::Element::Namespace(_) => None, // Namespaces don't have decl_file
            };

            elements_by_file.entry(decl_file).or_default().push(element);
        }

        // Normalize the compile unit path
        let cu_path_normalized = normalize_path(&cu.name);

        // Generate header files for each file in the file table
        for (file_idx, file_path) in cu.file_table.iter().enumerate() {
            let file_index = (file_idx + 1) as u64; // File table is 1-indexed

            // Skip the compile unit's own file - it will be generated later
            let header_path_normalized = normalize_path(file_path);
            if header_path_normalized == cu_path_normalized {
                continue;
            }

            if let Some(elements) = elements_by_file.get(&Some(file_index)) {
                if !elements.is_empty() {
                    let mut generator =
                        CodeGenerator::with_config(type_sizes.clone(), config.clone());

                    // Generate header content
                    generator.generate_header_comment(&cu.name, file_path);

                    // Generate elements for this header
                    generator.generate_elements(elements);

                    // Determine output path for header file
                    let output_path = output_dir.join(&header_path_normalized);

                    // Create parent directories if they don't exist
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::write(&output_path, generator.get_output())?;
                    println!("Generated: {} (from {})", output_path.display(), file_path);
                }
            }
        }

        // Generate main source file with elements from the compile unit itself
        // Find the file index for the compile unit (if it exists in the file table)
        let cu_file_index = cu
            .file_table
            .iter()
            .enumerate()
            .find(|(_, path)| normalize_path(path) == cu_path_normalized)
            .map(|(idx, _)| (idx + 1) as u64);

        // Collect elements: those with no decl_file + those declared in the CU file itself
        let mut main_elements: Vec<&types::Element> = Vec::new();

        // Add elements without decl_file
        if let Some(elems) = elements_by_file.get(&None) {
            main_elements.extend(elems.iter().copied());
        }

        // Add elements declared in the compile unit's own file
        if let Some(cu_idx) = cu_file_index {
            if let Some(elems) = elements_by_file.get(&Some(cu_idx)) {
                main_elements.extend(elems.iter().copied());
            }
        }

        if !main_elements.is_empty() || elements_by_file.is_empty() {
            let mut generator = CodeGenerator::with_config(type_sizes.clone(), config.clone());

            // Generate the compile unit with only the elements that belong to it
            if !main_elements.is_empty() {
                generator.generate_source_file(&cu.name, cu.producer.as_deref(), &main_elements);
            } else {
                // If all elements went to headers, still generate an empty source file
                generator.generate_compile_unit(cu);
            }

            let output_path = output_dir.join(&cu_path_normalized);

            // Create parent directories if they don't exist
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&output_path, generator.get_output())?;
            println!("Generated: {}", output_path.display());
        }
    }

    Ok(())
}

/// Normalize a file path by removing .. and . components
fn normalize_path(path: &str) -> String {
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
