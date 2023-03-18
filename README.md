# Jsorry: Json Parser

Simple recursive decent parser written in rust. This project was created for learning purposes and should not be used for real world use.

The only dependency for is itertools, the rest is standard Rust.

## How to use this project

Running the project:
- cargo run "your-input.json"

Testing the project:
- cargo test

Testing specific tests:
- cargo test parser::tests::test_array_element -- --exact