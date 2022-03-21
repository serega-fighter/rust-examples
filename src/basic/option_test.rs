
pub fn returnOption(x: i64) -> Option<i64> {
    if (x > 5) {
        return None
    } else {
        Some(x)
    }
}

fn main() {

    let result = returnOption(55);

    match returnOption(55) {
        Some(z) => println!("There is some: {}", z),
        None => println!("There is nothing")
    }

    match returnOption(2) {
        Some(z) => println!("There is some: {}", z),
        None => println!("There is nothing")
    }
}
