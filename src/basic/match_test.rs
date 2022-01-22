
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn retInt() -> i32 {
    5
}

fn retOptional() -> Option<String> {
    if retInt() > 4 {
        return Some(String::from("ABC"))
    } else {
        return None
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    match retOptional() {
        Some(x) => {
            println!("Got some {}", x)
        }
        None => {
            println!("Got none")
        }
    }

    println!("Value: {}", value_in_cents(Coin::Nickel))
}