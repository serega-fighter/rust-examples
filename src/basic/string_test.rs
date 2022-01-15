

fn main() {
    let hello = "Здравствуйте";
    let answer: &str = &hello[0..4];

    for c in hello.chars() {
        println!("{}", c);
    }

    println!("The value of x is: {}", answer);
}