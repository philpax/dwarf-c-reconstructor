mod element_processing;
mod error;
mod generator;
mod parser;
mod types;

use clap::Parser as ClapParser;
use element_processing::{group_elements_by_file, merge_namespaces, normalize_path};
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

    // Perform cross-CU matching across all object files in the archive.
    // This is necessary because class declarations and method definitions
    // may be in different object files (e.g., header.o vs implementation.o).
    parser::cross_cu_match_method_definitions(&mut all_compile_units);

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

    /// Enable all --no-* options and --shorten-int-types (minimal output)
    #[arg(long)]
    minimal: bool,

    /// Disable "//No line number" comments for elements without line numbers
    #[arg(long)]
    disable_no_line_comment: bool,

    /// Include "class " prefix in type references (only applies with --code-style=c)
    #[arg(long)]
    verbose_class_usage: bool,

    /// Code style for type prefixes: 'c' keeps struct/union/enum prefixes (default),
    /// 'c++' removes all prefixes. The 'class' prefix is only included with --verbose-class-usage in C mode.
    #[arg(long, value_name = "STYLE", default_value = "c")]
    code_style: String,

    /// Skip indentation inside namespaces (content starts at column 0)
    #[arg(long)]
    skip_namespace_indentation: bool,
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
        verbose_class_usage: args.verbose_class_usage,
        code_style: args.code_style.clone(),
        skip_namespace_indentation: args.skip_namespace_indentation,
    };

    // First pass: collect all header file elements from all compile units
    // Map from normalized path to (elements, original_path)
    let mut header_elements: HashMap<String, (Vec<types::Element>, String)> = HashMap::new();

    for cu in &compile_units {
        // Group elements by declaration file, properly handling namespaces
        // by splitting their children by decl_file
        let elements_by_file = group_elements_by_file(&cu.elements);

        // Normalize the compile unit path
        let cu_path_normalized = normalize_path(&cu.name);

        // Collect header file elements
        for (file_idx, file_path) in cu.file_table.iter().enumerate() {
            let file_index = (file_idx + 1) as u64; // File table is 1-indexed

            // Skip the compile unit's own file - it will be generated separately
            let header_path_normalized = normalize_path(file_path);
            if header_path_normalized == cu_path_normalized {
                continue;
            }

            if let Some(elements) = elements_by_file.get(&Some(file_index)) {
                if !elements.is_empty() {
                    let entry = header_elements
                        .entry(header_path_normalized)
                        .or_insert_with(|| (Vec::new(), file_path.clone()));
                    entry.0.extend(elements.iter().cloned());
                }
            }
        }
    }

    // Generate merged header files
    for (normalized_path, (elements, original_path)) in &header_elements {
        let mut generator = CodeGenerator::with_config(type_sizes.clone(), config.clone());

        // Generate header comment (use original path for display)
        generator.generate_header_comment_simple(original_path);

        // Merge namespaces with the same name and generate elements
        let merged_elements = merge_namespaces(elements.clone());
        let element_refs: Vec<&types::Element> = merged_elements.iter().collect();
        generator.generate_elements(&element_refs);

        // Determine output path for header file
        let output_path = output_dir.join(normalized_path);

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, generator.get_output())?;
        println!(
            "Generated: {} (from {})",
            output_path.display(),
            original_path
        );
    }

    // Second pass: generate source files for each compile unit
    for cu in &compile_units {
        // Group elements by declaration file, properly handling namespaces
        // by splitting their children by decl_file
        let elements_by_file = group_elements_by_file(&cu.elements);

        // Normalize the compile unit path
        let cu_path_normalized = normalize_path(&cu.name);

        // Generate main source file with elements from the compile unit itself
        // Find the file index for the compile unit (if it exists in the file table)
        let cu_file_index = cu
            .file_table
            .iter()
            .enumerate()
            .find(|(_, path)| normalize_path(path) == cu_path_normalized)
            .map(|(idx, _)| (idx + 1) as u64);

        // Collect elements: those with no decl_file + those declared in the CU file itself
        // We collect owned elements so we can merge namespaces with the same name
        let mut main_elements_owned: Vec<types::Element> = Vec::new();

        // Add elements without decl_file
        if let Some(elems) = elements_by_file.get(&None) {
            main_elements_owned.extend(elems.iter().cloned());
        }

        // Add elements with decl_file = 0 (some DWARF producers use 0 for the CU file)
        if let Some(elems) = elements_by_file.get(&Some(0)) {
            main_elements_owned.extend(elems.iter().cloned());
        }

        // Add elements declared in the compile unit's own file
        if let Some(cu_idx) = cu_file_index {
            // Avoid double-counting if cu_idx is 0
            if cu_idx != 0 {
                if let Some(elems) = elements_by_file.get(&Some(cu_idx)) {
                    main_elements_owned.extend(elems.iter().cloned());
                }
            }
        }

        // Merge namespaces with the same name to avoid duplicate namespace blocks
        let main_elements_merged = merge_namespaces(main_elements_owned);
        let main_elements: Vec<&types::Element> = main_elements_merged.iter().collect();

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
