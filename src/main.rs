use std::io::{self, Write}; // 1. Import the Write trait

fn main() {
    print!("Hello, World!"); // 2. Print "Hello, World!" like a Rust beginner example
    
    // 3. Flush stdout so the text displays immediately
    io::stdout().flush().unwrap(); 
}
