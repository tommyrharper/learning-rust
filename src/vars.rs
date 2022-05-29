// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Tom";
  let mut age = 25;

  println!("My name is {} and I am {}", name, age);

  age = 26;

  println!("My name is {} and I am {}", name, age);
}