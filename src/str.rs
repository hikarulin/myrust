pub fn strtest() {
    let hello = String::from("नमस्ते");
    let chars = hello.chars();
    println!("len: {}", hello.len());
    for (_i,c) in chars.enumerate() {
        print!("{}", c);
    }
}