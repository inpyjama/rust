fn main() {
    let pi: f64 = 3.14; // Rust infers the type as f64 (double)
    let whole_pi: u32 = pi; // Error! Type mismatch (f64 to i32)
    println!("Whole part of pi: {}", whole_pi);
}
