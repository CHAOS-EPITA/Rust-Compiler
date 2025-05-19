# Mini Rust Compiler

This project is a simple Rust compiler that translates Rust source code into C code and compiles it into an executable. It demonstrates the process of lexing, parsing, and code generation.

## Project Structure

- **Cargo.toml**: Configuration file for the Rust project, specifying package details and dependencies.
- **src/main.rs**: Entry point of the application, handling command-line arguments and orchestrating the compilation process.
- **src/lexer.rs**: Defines the `Lexer` struct and methods for tokenizing source code, including token types.
- **src/parser.rs**: Defines the `Parser` struct and methods for parsing tokens into an Abstract Syntax Tree (AST).
- **src/code_generator.rs**: Contains the `CodeGenerator` struct and methods for generating C code from the AST and compiling it.
- **src/error_handler.rs**: Defines the `ErrorHandler` struct for reporting errors during compilation with formatted messages.
- **src/lib.rs**: Library entry point for exposing functionality from other modules.
- **examples/test.rs**: Example Rust code demonstrating basic arithmetic operations and function definitions.
- **examples/hello_world.rs**: Simple "Hello, World!" program written in Rust.
- **tests/integration_tests.rs**: Integration tests verifying the compiler's processing of various Rust source files.
- **tests/compiler_tests.rs**: Unit tests for the compiler's components, ensuring functionality of the lexer, parser, and code generator.

## Building the Project

To build the project, ensure you have Rust and Cargo installed. Then, run the following command in the project directory:

```
cargo build
```

## Running the Compiler

To compile a Rust source file using the mini Rust compiler, use the following command:

```
cargo run <path_to_rust_file>
```

Replace `<path_to_rust_file>` with the path to the Rust file you want to compile.

## Running Tests

To run the tests for the mini Rust compiler, use the following command:

```
cargo test
```

This will execute both integration and unit tests to ensure the compiler functions correctly.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.