fn main() {
    // Basic Printing
    println!("Hello, Rust World!");  // Prints with a newline
    print!("This stays on the same line "); // No newline
    println!("...and now we're on a new line.");

    // Formatting with placeholders
    let name = "Ferris🦀";
    let age = 3;
    println!("My name is {} and I am {} years old.", name, age);

    // Detailed formatting control
    println!("{:?}", (12, true, "hello")); // Debug output
    println!("{:#?}", (12, true, "hello")); // Pretty-printed debug output 
    println!("Number: {}", 42);  
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10); 

    // Formatting with named arguments
    println!("{name} says {greeting}.", name="Alice", greeting="Howdy");

    // Padding and alignment
    println!("Right-aligned: {:>10}", "hello");   // Pad with spaces on the left
    println!("Left-aligned: {:<10}", "hello");    // Pad with spaces on the right
    println!("Centered:     {:^10}", "hello");   // Pad with spaces on both sides

    // Precision for floating-point numbers
    println!("Pi with 3 decimals: {:.3}", 3.14159); 
}