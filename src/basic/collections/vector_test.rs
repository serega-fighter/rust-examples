
fn main() {
    let mut v: Vec<String> = Vec::new();

    v.push(String::from("A"));
    v.push(String::from("B"));
    v.push(String::from("C"));

    // Iterate with index
    for (pos, e) in v.iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
    }

    let sz: usize = v.len();
    println!("Size is {}", sz);

    for i in 0..sz {
        println!("Element is {}", i);
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