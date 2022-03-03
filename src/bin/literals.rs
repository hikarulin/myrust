fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `1u8` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `2u32` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `3f32` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `1` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `1.0` in bytes: {}", std::mem::size_of_val(&f));
}