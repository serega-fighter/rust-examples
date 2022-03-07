
struct S<'a> {
    r: &'a i32
}

fn main() {
    let x = 5;
    let z = S {
        r: &x
    };
}