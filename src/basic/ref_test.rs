
pub fn change_str(s: &mut String) {
    s.push_str("AAC");
}

fn main() {
    let mut s = String::from("AABBC");

    change_str(&mut s);
    println!("The value of x is: {}", s);
}