
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


    /*
    The compiler can optimize this kind of match using a jump table, just like
    a switch statement in C++. A similar optimization is applied when each arm of a
    match produces a constant value. In that case, the compiler builds an array of
    those values, and the match is compiled into an array access. Apart from a bounds
    check, there is no branching at all in the compiled code.
    */
    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        3 => println!("User log"),
        _ => println!("Unrecognized Error {}", code)
    }
}