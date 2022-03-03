#![allow(unreachable_code)]

fn main() {
    let mut inner_cnt = 0;
    'outer: loop {
        println!("> Entered the outer loop");

        'inner: loop {
            inner_cnt = inner_cnt + 1;
            println!(">> Entered the inner loop, cnt:{}",inner_cnt);
            // This breaks the outer loop
            if inner_cnt > 1 {
                break 'outer;
            }
            println!("<< inner loop last statement");
            break
        }
        println!("< outer loop last statement");
    }

    println!("Exited");
}
