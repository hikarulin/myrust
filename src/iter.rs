
pub fn iterator_demonstration() {
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
}