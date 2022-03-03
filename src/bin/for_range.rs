fn main() {
    let mut arr = vec![];
    // 2..4 == [2,4)
    for n in 2..4 {
        arr.push(n);
    }
    assert_eq!(arr,vec![2,3]);
    arr.clear();
    // 2..=4 == [2,4]
    for n in 2..=4 {
        arr.push(n);
    }
    assert_eq!(arr,vec![2,3,4]);
    // 这里默认调用collection.into_iter(self)
    // 因此该方法后不能在借用arr
    // for x in arr {
    //     println!("val:{}",x);
    // }
    for x in arr.iter() {
        println!("val:{}",x);
    }
    assert_eq!(arr,vec![2,3,4]);
    for x in arr.iter_mut() {
        *x *= 2;
    }
    assert_eq!(arr,vec![4,6,8]);

}