### cargo check vs cargo build:
- `cargo build` compiles and outputs an executable (on windows)
    - outputs to ~/target/debug/
- `cargo check` runs the compiler to ensure the app builds
- cargo check is faster and the preferred way to test compilation

### cargo building:
- `cargo build --release` to compile the project w/ optimizations. Outputs to ~/target/release (similar to .net)

### checking multiple files
- `cargo check --workspace` to check multiple files, not must main.rs


### start back [here](https://doc.rust-lang.org/book/ch03-05-control-flow.html)