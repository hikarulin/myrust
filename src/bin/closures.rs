
fn main() {
    let fn1 = |x|x+1;
    println!("{}",fn1(1));
    // let floatv = fn1(2.0);// 编译失败,第一次调用fn1时,参数已经确定为integer
}