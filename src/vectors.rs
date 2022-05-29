// Vectors - Resizable Vectors

use std::mem;

pub fn run() {
  let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get Vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
