use std::ops::Range;
use std::ops::RangeFrom;

pub fn main() {
    let a: Range<i32> = 1..5;
    println!("{:?}", a);

    for x in a {
        println!("Next value is {}", x);
    }

    let b: RangeFrom<i32> = 1..;
    println!("{:?}", b);
}