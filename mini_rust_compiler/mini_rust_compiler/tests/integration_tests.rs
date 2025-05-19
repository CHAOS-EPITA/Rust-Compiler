// This file contains integration tests for the mini Rust compiler. 
// It verifies that the compiler correctly processes various Rust source files.

#[cfg(test)]
mod tests {
    use std::process::Command;

    fn run_compiler(file: &str) -> bool {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(file)
            .output()
            .expect("Failed to execute compiler");

        output.status.success()
    }

    #[test]
    fn test_example_test() {
        assert!(run_compiler("examples/test.rs"));
    }

    #[test]
    fn test_hello_world() {
        assert!(run_compiler("examples/hello_world.rs"));
    }

    #[test]
    fn test_invalid_code() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("examples/invalid.rs")
            .output()
            .expect("Failed to execute compiler");

        assert!(!output.status.success());
    }
}