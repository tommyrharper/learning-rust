# Rust Tutorial

## Hello World

```rust
fn() main {
  println!("Hello World!");
}
```

- Compile: `rustc ./hello.rc`
- Run: `./hello`


## Building a project

- Create new project: `cargo init`
- Compile and run: `cargo run`
  - Compiles into `./target/debug/[project-name]`
    - In this case: `./target/debug/traversy-rust`
    - You can then also run the code in this manner:
      - `./target/debug/[project-name]` `./target/debug/traversy-rust`
- Prod build: `cargo build --release`
  - Compiles into `./target/release/[project-name]`

