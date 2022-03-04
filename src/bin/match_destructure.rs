fn main() {
    match_tupple();
    match_array();
}

fn match_tupple() {
    let tuple = ("shindou", 18, vec!["basketball"]);
    match tuple {
        ("king",y,z) => {
            println!("It's king!");
        }
        (name, age, interests) if age <= 18 => {
            println!("A teenager {} aged:{}, interests:{:?}", name, age, interests);
        }
        (name, age, _) => {
            println!("nobody cares");
        }
    }
}

fn match_array() {
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        [-1, tail @ ..] => println!(
            "array[0] = -1, with tails:{:?}",
            tail
        ),
        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}