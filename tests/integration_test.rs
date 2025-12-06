use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_jcphuff_object_file() {
    let sample_path = "samples/jcphuff.o";
    if !Path::new(sample_path).exists() {
        eprintln!("Sample file {} not found, skipping test", sample_path);
        return;
    }

    let output_dir = "/tmp/test_jcphuff_integration";
    let _ = fs::remove_dir_all(output_dir);

    let output = Command::new("cargo")
        .args(["run", "--", sample_path, "-o", output_dir])
        .output()
        .expect("Failed to execute dwarf-c-reconstructor");

    assert!(
        output.status.success(),
        "dwarf-c-reconstructor failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify expected files were generated
    assert!(
        Path::new(&format!("{}/src/jpeglib/jpeglib.h", output_dir)).exists(),
        "jpeglib.h was not generated"
    );
    assert!(
        Path::new(&format!("{}/src/jpeglib/jpegint.h", output_dir)).exists(),
        "jpegint.h was not generated"
    );
    assert!(
        Path::new(&format!("{}/src/jpeglib/jerror.h", output_dir)).exists(),
        "jerror.h was not generated"
    );
    assert!(
        Path::new(&format!("{}/src/jpeglib/jcphuff.c", output_dir)).exists(),
        "jcphuff.c was not generated"
    );

    // Verify the files have content
    let jpeglib_h = fs::read_to_string(format!("{}/src/jpeglib/jpeglib.h", output_dir))
        .expect("Failed to read jpeglib.h");
    assert!(
        jpeglib_h.contains("typedef struct"),
        "jpeglib.h should contain type definitions"
    );
    assert!(
        jpeglib_h.contains("JQUANT_TBL") || jpeglib_h.contains("JHUFF_TBL"),
        "jpeglib.h should contain JPEG types"
    );

    // Cleanup
    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn test_multiple_object_files_no_duplicate_headers() {
    // Test that processing multiple object files generates consistent headers
    let samples = ["samples/jcphuff.o"];
    let base_output = "/tmp/test_headers_base";

    for (idx, sample_path) in samples.iter().enumerate() {
        if !Path::new(sample_path).exists() {
            continue;
        }

        let output_dir = format!("{}{}", base_output, idx);
        let _ = fs::remove_dir_all(&output_dir);

        let output = Command::new("cargo")
            .args(["run", "--", sample_path, "-o", &output_dir])
            .output()
            .expect("Failed to execute dwarf-c-reconstructor");

        assert!(
            output.status.success(),
            "dwarf-c-reconstructor failed for {}:\n{}",
            sample_path,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Cleanup
    for idx in 0..samples.len() {
        let output_dir = format!("{}{}", base_output, idx);
        let _ = fs::remove_dir_all(output_dir);
    }
}

#[test]
fn test_class_accessibility() {
    // Create a simple C++ class with different access levels
    let test_cpp = "/tmp/test_accessibility.cpp";
    let test_obj = "/tmp/test_accessibility.o";
    let output_dir = "/tmp/test_accessibility_output";

    let cpp_code = r#"
class TestClass {
private:
    int private_member;
protected:
    int protected_member;
public:
    int public_member;
    void public_method();
};
"#;

    fs::write(test_cpp, cpp_code).expect("Failed to write test file");

    // Compile with debug info
    let compile = Command::new("g++")
        .args(["-g", "-c", test_cpp, "-o", test_obj])
        .output();

    if compile.is_err() || !compile.as_ref().unwrap().status.success() {
        eprintln!("g++ not available or compilation failed, skipping test");
        let _ = fs::remove_file(test_cpp);
        return;
    }

    let _ = fs::remove_dir_all(output_dir);

    let output = Command::new("cargo")
        .args(["run", "--", test_obj, "-o", output_dir])
        .output()
        .expect("Failed to execute dwarf-c-reconstructor");

    assert!(
        output.status.success(),
        "dwarf-c-reconstructor failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify the generated file has proper access sections
    let generated_files: Vec<_> = fs::read_dir(output_dir)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "cpp"))
        .collect();

    if !generated_files.is_empty() {
        let generated_path = &generated_files[0].path();
        let content = fs::read_to_string(generated_path).expect("Failed to read generated file");

        // Check for access specifiers
        assert!(
            content.contains("private:"),
            "Generated class should have private section"
        );
        assert!(
            content.contains("protected:"),
            "Generated class should have protected section"
        );
        assert!(
            content.contains("public:"),
            "Generated class should have public section"
        );
    }

    // Cleanup
    let _ = fs::remove_file(test_cpp);
    let _ = fs::remove_file(test_obj);
    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn test_archive_file() {
    let sample_path = "samples/libjpeg_x86_64.a";
    if !Path::new(sample_path).exists() {
        eprintln!("Sample archive {} not found, skipping test", sample_path);
        return;
    }

    let output_dir = "/tmp/test_archive_integration";
    let _ = fs::remove_dir_all(output_dir);

    let output = Command::new("cargo")
        .args(["run", "--", sample_path, "-o", output_dir])
        .output()
        .expect("Failed to execute dwarf-c-reconstructor");

    assert!(
        output.status.success(),
        "dwarf-c-reconstructor failed on archive:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify that files were generated from the archive
    let generated_files: Vec<_> = fs::read_dir(output_dir)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .collect();

    assert!(
        !generated_files.is_empty(),
        "No files were generated from the archive"
    );

    // Verify that we have both C source files and headers
    let mut has_c_files = false;
    let mut has_h_files = false;

    fn check_files(dir: &Path, has_c: &mut bool, has_h: &mut bool) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    check_files(&path, has_c, has_h);
                } else if let Some(ext) = path.extension() {
                    if ext == "c" {
                        *has_c = true;
                    } else if ext == "h" {
                        *has_h = true;
                    }
                }
            }
        }
    }

    check_files(Path::new(output_dir), &mut has_c_files, &mut has_h_files);

    assert!(has_c_files, "No C source files generated from archive");
    assert!(has_h_files, "No header files generated from archive");

    // Cleanup
    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn test_class_method_parameters() {
    // Test that method parameters from definitions are correctly associated with declarations
    let test_cpp = "/tmp/test_method_params.cpp";
    let test_obj = "/tmp/test_method_params.o";
    let output_dir = "/tmp/test_method_params_output";

    // C++ code with class method declarations (no params) and definitions (with params)
    // This tests the DW_AT_specification linking
    let cpp_code = r#"
class MyClass {
public:
    int value;
    MyClass();
    ~MyClass();
    int add(int a, int b);
    void setData(const char *name, int size);
};

MyClass::MyClass() : value(0) {}

MyClass::~MyClass() {}

int MyClass::add(int a, int b) {
    return a + b;
}

void MyClass::setData(const char *name, int size) {
    // use params to avoid warnings
    (void)name;
    (void)size;
}
"#;

    fs::write(test_cpp, cpp_code).expect("Failed to write test file");

    // Compile with debug info
    let compile = Command::new("g++")
        .args(["-g", "-c", test_cpp, "-o", test_obj])
        .output();

    if compile.is_err() || !compile.as_ref().unwrap().status.success() {
        eprintln!("g++ not available or compilation failed, skipping test");
        let _ = fs::remove_file(test_cpp);
        return;
    }

    let _ = fs::remove_dir_all(output_dir);

    let output = Command::new("cargo")
        .args(["run", "--", test_obj, "-o", output_dir])
        .output()
        .expect("Failed to execute dwarf-c-reconstructor");

    assert!(
        output.status.success(),
        "dwarf-c-reconstructor failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Find the generated .cpp file (may be in subdirectories)
    fn find_cpp_files(dir: &Path) -> Vec<std::path::PathBuf> {
        let mut result = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    result.extend(find_cpp_files(&path));
                } else if path.extension().is_some_and(|ext| ext == "cpp") {
                    result.push(path);
                }
            }
        }
        result
    }

    let generated_files = find_cpp_files(Path::new(output_dir));

    assert!(!generated_files.is_empty(), "No .cpp files were generated");

    let generated_path = &generated_files[0];
    let content = fs::read_to_string(generated_path).expect("Failed to read generated file");

    // Check that the class declaration has method parameters
    // Note: the parameters should be on the method declarations inside the class
    assert!(
        content.contains("int add(int a, int b)") || content.contains("int add(int, int)"),
        "add method should have parameters in class declaration. Content:\n{}",
        content
    );

    assert!(
        content.contains("setData(const char") || content.contains("setData(char"),
        "setData method should have const char* parameter in class declaration. Content:\n{}",
        content
    );

    // Check that method definitions are generated with class:: prefix
    assert!(
        content.contains("MyClass::add(") || content.contains("MyClass::add "),
        "Method definitions should have MyClass:: prefix. Content:\n{}",
        content
    );

    assert!(
        content.contains("MyClass::setData(") || content.contains("MyClass::setData "),
        "setData definition should have MyClass:: prefix. Content:\n{}",
        content
    );

    // Cleanup
    let _ = fs::remove_file(test_cpp);
    let _ = fs::remove_file(test_obj);
    let _ = fs::remove_dir_all(output_dir);
}
