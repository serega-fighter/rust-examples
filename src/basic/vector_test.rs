
fn main() {
    let mut v: Vec<String> = Vec::new();

    v.push(String::from("A"));
    v.push(String::from("B"));
    v.push(String::from("C"));
    v.push(String::from("D"));

    for (pos, e) in v.iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i = String::from("Z");
    }

    for i in &v {
        println!("{}", i);
    }
}