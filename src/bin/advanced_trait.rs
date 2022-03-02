use std::ops::Add;

mod concurrent;
mod duotai;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Millimeters) -> Millimeters {
        return Millimeters(self.0+rhs.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        return Millimeters(self.0+rhs.0*1000)
    }
}

fn main() {
    let mill1 = Millimeters(100);
    let mill2 = Millimeters(200);
    let meter1 = Meters(100);

    let mill3 = mill1 + mill2;
    println!("{:?}",mill3);
    let meter2 = mill3 + meter1;
    println!("{:?}",meter2);
}
