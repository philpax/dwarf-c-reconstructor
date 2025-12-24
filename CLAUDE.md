# CLAUDE.md

Reconstructs C/C++ type definitions and function signatures from DWARF debugging information in ELF files.

## Development

- Format code with `cargo fmt` before committing
- Run `cargo clippy` to catch common issues
- Run `cargo test` to execute unit tests

## Manual Testing

Test against all sample files in `samples/` to verify output quality. Use subagents to test each file individually in parallel:

```bash
cargo run -- <sample_file> -o /tmp/<output_dir>
```

Review the generated output files to confirm they contain sensible C/C++ type definitions and function declarations. Ensure all samples are tested to catch architecture-specific or format-specific issues.
