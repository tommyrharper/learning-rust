pub fn run() {
  // Basic formatting
  println!("Hello from the print.rs file! {} {}", 1, "interpolated");

  // Positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Tom", "London", "code");

  // Named arguments
  println!("{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );

  // Placeholder traits
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
  println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
