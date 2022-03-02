pub fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(6) {
        Some(v) => println!("The seventh element is {}", v),
        None => println!("There is no seventh element."),
    }
}

pub fn match_err() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // match v.get(2) { //patterns `Some(&i32::MIN..=2_i32)` and `Some(&4_i32..=i32::MAX)` not covered
    //     Some(3) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
}