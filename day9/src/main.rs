fn main() {
    // Array creation with automatic size inference
    let mut my_numbers = [1, 2, 3, 4, 5];

    // Array creation with explicit type and size
    let weekdays: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    // Accessing elements using indexing
    println!("The first number: {}", my_numbers[0]);
    println!("The third weekday: {}", weekdays[2]);

    // Iterating over an array
    for number in my_numbers {
        println!("Number: {}", number);
    }

    // Unlike C arrays are bound checked in rust
    // my_numbers[6] = 6; // This would cause a compile-time error

    // Arrays are types and cannot be directly assigned
    let another_array = [10, 20, 30];
    // my_numbers = another_array;  // This would cause a compile-time error due to mismatched types
}
