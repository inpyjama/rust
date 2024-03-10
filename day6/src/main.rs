fn main() {
    // Variable Declaration and Type Inference
    let my_number = 10;         // Type inferred as i32
    let mut my_string = "Hello"; // Type inferred as &str

    println!("Initial values - number: {}, string: {}", my_number, my_string);

    // Mutability 
    my_string = "Hello, world!"; 
    println!("Modified string: {}", my_string);

    // Variables must be initialized
    let unassigned_variable: u32; // This would cause a compile-time error

    // Try uncommenting the following line to see the error:
    // println!("Unassigned variable: {}", unassigned_variable);
}