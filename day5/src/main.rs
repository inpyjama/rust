use std::mem::size_of;

fn main() {
    println!("Data type sizes in Rust:");

    println!("bool:        {} bytes", size_of::<bool>());
    println!("char:        {} bytes", size_of::<char>());
    println!("i8:          {} bytes", size_of::<i8>());
    println!("i16:         {} bytes", size_of::<i16>());
    println!("i32:         {} bytes", size_of::<i32>());
    println!("i64:         {} bytes", size_of::<i64>());
    println!("u8:          {} bytes", size_of::<u8>());
    println!("u16:         {} bytes", size_of::<u16>());
    println!("u32:         {} bytes", size_of::<u32>());
    println!("u64:         {} bytes", size_of::<u64>());
    println!("f32:         {} bytes", size_of::<f32>());
    println!("f64:         {} bytes", size_of::<f64>());
    println!("usize:       {} bytes", size_of::<usize>());
    println!("isize:       {} bytes", size_of::<isize>());
}