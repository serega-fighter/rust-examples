

fn main() {
    let hello = "Здравствуйте";
    let answer: &str = &hello[0..4];
    for c in hello.chars() {
        println!("{}", c);
    }
    println!("The value of x is: {}", answer);


    // Split

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);

}