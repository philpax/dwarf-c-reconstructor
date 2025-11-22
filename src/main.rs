mod generator;
mod parser;
mod types;

use clap::Parser as ClapParser;
use generator::CodeGenerator;
use parser::DwarfParser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// DWARF C reconstructor - generates C++ code from DWARF debugging information
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(value_name = "ELF_FILE")]
    file_path: String,

    /// Output directory for generated files
    #[arg(short, long, default_value = "output")]
    output_dir: String,
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

    for cu in &compile_units {
        let mut generator = CodeGenerator::with_type_sizes(type_sizes.clone());
        generator.generate_compile_unit(cu);

        // Determine output file path, preserving directory structure
        let output_rel_path = if cu.name.is_empty() {
            "unknown.c".to_string()
        } else {
            // Normalize and clean up the path
            let path = Path::new(&cu.name);

            // Convert to a clean relative path, removing .. and . components
            let mut components = Vec::new();
            for component in path.components() {
                match component {
                    std::path::Component::Normal(c) => {
                        components.push(c.to_str().unwrap_or("unknown"));
                    }
                    std::path::Component::ParentDir => {
                        // Skip parent directory references for cleaner output
                        if !components.is_empty() {
                            components.pop();
                        }
                    }
                    std::path::Component::CurDir => {
                        // Skip current directory references
                    }
                    _ => {}
                }
            }

            if components.is_empty() {
                "unknown.c".to_string()
            } else {
                components.join("/")
            }
        };

        let output_path = output_dir.join(&output_rel_path);

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, generator.get_output())?;
        println!("Generated: {}", output_path.display());
    }

    Ok(())
}
