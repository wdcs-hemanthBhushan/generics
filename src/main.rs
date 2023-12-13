use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
#[derive(Debug)]
struct Person {
    x: i32,
}
fn check<T: Display>(value: T) -> T {
    println!("{}", value);
    value
}

fn check2<T: Debug>(value: T) -> T {
    println!("{:?}", value);
    value
}

fn multiple<T: Display, U: Display + PartialOrd>(x: T, y: U, z: U) {
    println!("{}x{}x{}X{}>{},{}", x, y, z, y, z, y > z);
}

fn different<T, U, V>(x: T, y: U, z: V)
where
    T: Debug,
    U: Debug,
    V: PartialOrd + Debug,
{
    println!("{:?},{:?},{:?}", x, y, z);
}
fn main() {
    check(23);
    check2(34);
    multiple("hemanht", 23, 455);

    let y = Person { x: 23 };
    different(23, y, 4)
}
