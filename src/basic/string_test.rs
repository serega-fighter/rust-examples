

fn main() {
    let hello = "Здравсвуйте";
    let answer: &str = &hello[0..4];
    for c in answer.chars() {
        println!("{}", c);
    }

    // From String
    let x: String = String::from("ABCCDEF");
    println!("Just string: {}", x);

    let borrowed: &str = x.as_str();
    println!("Just borrowed string: {}", borrowed);

    let createdAgain: String = String::from(borrowed);
    println!("Just create from borrow: {}", createdAgain);

    // Split
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);

}