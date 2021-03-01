pub fn match1() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
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