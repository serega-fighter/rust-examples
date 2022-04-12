use std::ops::Range;
use std::ops::RangeFrom;

pub fn main() {
    let a: Range<i32> = 1..100;
    println!("{:?}", a);

    let b: RangeFrom<i32> = 1..;
    println!("{:?}", b);
}