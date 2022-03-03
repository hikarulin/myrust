fn main() {
    let mut cnt = 0;
    let sum = loop {
        cnt += 1;
        if cnt > 10 {
            break cnt;
        }
    };
    println!("{}",cnt);
}