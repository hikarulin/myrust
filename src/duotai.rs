pub trait Animal {
    fn eat(&self);
}

pub struct Lion {}

impl Animal for Lion {
    fn eat(&self) {
        println!("lion eating");
    }
}

pub struct Tiger {}

impl Animal for Tiger{
    fn eat(&self) {
        println!("tiger eating");
    }
}

// 该方法编译失败
// pub fn newAnimal(s: &String) -> impl Animal {
//     return if s == "lion" {
//         Lion {}
//     } else {
//         Tiger {}
//     }
// }

pub fn run() {
    let lion = newAnimal("lion");
    lion.eat();
    let tiger = newAnimal("tiger");
    tiger.eat();
}

// dyn 表示动态分发
// 相比静态分发会有部分性能损失
pub fn newAnimal(s: &str) -> Box<dyn Animal> {
    return if s == "lion" {
        Box::new(Lion {})
    } else {
        Box::new(Tiger {})
    }
}