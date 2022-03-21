
pub fn return_option(x: i64) -> Option<i64> {
    if (x > 5) {
        return None
    } else {
        Some(x)
    }
}

fn main() {

    match return_option(55) {
        Some(z) => println!("There is some: {}", z),
        None => println!("There is nothing")
    }

    match return_option(2) {
        Some(z) => println!("There is some: {}", z),
        None => println!("There is nothing")
    }

    let result = return_option(55);
    println!("is_some: {}", result.is_some());

    let value = result.expect("There is no value");
}
