mod error;
mod generator;
mod parser;
mod types;

use clap::Parser as ClapParser;
use error::Result;
use generator::{CodeGenConfig, CodeGenerator};
use object::read::archive::ArchiveFile;
use object::Object;
use parser::DwarfParser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parse a single object file's DWARF data
fn parse_object_file(data: &[u8]) -> Result<Vec<types::CompileUnit>> {
    let mut parser = DwarfParser::new(data)?;
    parser.parse()
}

/// Detect pointer size from an object file (returns 4 for 32-bit, 8 for 64-bit)
fn detect_pointer_size(data: &[u8]) -> u64 {
    if let Ok(obj) = object::File::parse(data) {
        if obj.is_64() {
            return 8;
        }
    }
    4 // Default to 32-bit
}

/// Detect pointer size from an archive by checking the first object member
fn detect_pointer_size_from_archive(archive: &ArchiveFile<'_>, archive_data: &[u8]) -> u64 {
    for member in archive.members().flatten() {
        if let Ok(member_data) = member.data(archive_data) {
            if object::File::parse(member_data).is_ok() {
                return detect_pointer_size(member_data);
            }
        }
    }
    4 // Default to 32-bit
}

/// Parse an archive file and process all object file members
fn parse_archive(archive: ArchiveFile<'_>, archive_data: &[u8]) -> Result<Vec<types::CompileUnit>> {
    let mut all_compile_units = Vec::new();

    // Collect all member data into owned Vec<u8> to ensure proper lifetime
    let mut member_data_storage: Vec<Vec<u8>> = Vec::new();

    for member_result in archive.members() {
        let member = member_result?;
        let member_data = member.data(archive_data)?;

        // Skip non-object files (like symbol tables)
        if object::File::parse(member_data).is_ok() {
            member_data_storage.push(member_data.to_vec());
        }
    }

    // Now parse each member's DWARF data
    for member_data in &member_data_storage {
        match parse_object_file(member_data) {
            Ok(mut compile_units) => {
                all_compile_units.append(&mut compile_units);
            }
            Err(e) => {
                // Some members might not have DWARF data, that's okay
                eprintln!("Warning: Failed to parse archive member: {}", e);
            }
        }
    }

    Ok(all_compile_units)
}

/// DWARF C reconstructor - generates C++ code from DWARF debugging information
#[derive(ClapParser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file or archive to analyze
    #[arg(value_name = "FILE")]
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

    /// Enable all --no-* options (minimal output with no addresses, offsets, or prototypes)
    #[arg(long)]
    minimal: bool,

    /// Disable "//No line number" comments for elements without line numbers
    #[arg(long)]
    disable_no_line_comment: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read file data
    let file_data = fs::read(&args.file_path)?;
    let file_data_slice: &[u8] = &file_data;

    // Detect pointer size and parse file
    let (compile_units, pointer_size) = if let Ok(archive) = ArchiveFile::parse(file_data_slice) {
        // It's an archive file - detect pointer size and process each member
        let ptr_size = detect_pointer_size_from_archive(&archive, file_data_slice);
        // Re-parse archive for member iteration (iterators are consumed)
        let archive = ArchiveFile::parse(file_data_slice)?;
        (parse_archive(archive, file_data_slice)?, ptr_size)
    } else {
        // It's a regular object file
        let ptr_size = detect_pointer_size(file_data_slice);
        (parse_object_file(file_data_slice)?, ptr_size)
    };

    // Collect all type sizes from all compile units
    let mut type_sizes = HashMap::new();
    for cu in &compile_units {
        CodeGenerator::collect_type_sizes_from_elements(&mut type_sizes, &cu.elements);
    }

    // Generate code for each compile unit
    let output_dir = Path::new(&args.output_dir);
    fs::create_dir_all(output_dir)?;

    // Create config from command-line args
    // If --minimal is set, enable all --no-* options and shorten_int_types
    let config = CodeGenConfig {
        shorten_int_types: args.shorten_int_types || args.minimal,
        no_function_addresses: args.no_function_addresses || args.minimal,
        no_offsets: args.no_offsets || args.minimal,
        no_function_prototypes: args.no_function_prototypes || args.minimal,
        pointer_size,
        disable_no_line_comment: args.disable_no_line_comment,
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
                types::Element::TypedefAlias(t) => t.decl_file,
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
