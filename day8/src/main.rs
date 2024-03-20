fn circle_properties(radius: f64) -> (f64, f64) {
    let area = std::f64::consts::PI * radius.powi(2);
    let circumference = 2.0 * std::f64::consts::PI * radius;
    (area, circumference)
}

fn main() {
    // Tuple creation
    let my_tuple: (i32, f64, String) = (10, 3.14, "Hello".to_string());

    // Accessing elements
    println!("First element (integer): {}", my_tuple.0);
    println!("Second element (float): {}", my_tuple.1);
    println!("Third element (string): {}", my_tuple.2);

    // Destructuring
    let (area, circumference) = circle_properties(5.0);
    println!("Area: {}, Circumference: {}", area, circumference);

    // Tuple elements can be mutated:
    let mut another_tuple = (1, vec![2, 3, 4]); 
    another_tuple.1.push(5); // Modifying the vector within the tuple

    println!("Modified tuple: {:?}", another_tuple);
}
