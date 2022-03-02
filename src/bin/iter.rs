
pub fn main() {
    let mut v1 = vec![1, 2, 3];
    let mut iter1 = v1.iter();
    assert_eq!(iter1.next(), Some(&1));
    assert_eq!(iter1.next(), Some(&2));
    assert_eq!(iter1.next(), Some(&3));
    assert_eq!(iter1.next(), None);

    let iter2 = v1.iter_mut();
    for elem in iter2 {
        *elem *= 2;
    }
    assert_eq!(v1, &[2, 4, 6]);

    let x:i32 = v1.iter().sum();//不加类型编译无法通过
    println!("sum: {}",x);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    let arr = 12..15;
    println!("range:{:?}",arr);
    for a in arr {
        println!("{}",a);
    }

}