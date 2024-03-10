fn main() {
    // Tuple creation
    let my_tuple: (i32, f64, String) = (10, 3.14, "Hello".to_string());

    // Accessing elements
    println!("First element (integer): {}", my_tuple.0);
    println!("Second element (float): {}", my_tuple.1);
    println!("Third element (string): {}", my_tuple.2);

    // Destructuring
    let (x, y, z) = my_tuple;
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    // Mutability (with a twist!)
    // Tuples themselves are immutable, but elements can be mutable if they are:
    let mut another_tuple = (1, vec![2, 3, 4]); // Vec is mutable
    another_tuple.1.push(5); // Modifying the vector within the tuple

    println!("Modified tuple: {:?}", another_tuple);
}