
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let data = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let d = Box::new(5);
    let x = d;
    println!("{}", *x);
}
