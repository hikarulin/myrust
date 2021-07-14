

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // let mut number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("{:?} The largest number is {}", number_list, result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // return;
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T>(list: &[T]) -> &T
    where T: PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}