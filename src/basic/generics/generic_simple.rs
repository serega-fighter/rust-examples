
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

pub fn main() {
    println!("Min of {} and {} is {}", 5, 6, min(5, 6));
}