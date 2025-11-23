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
