
struct S<'a> {
    r: &'a i32
}

struct S2<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

fn main() {
    let x = 5;
    let r;
    {
        let y = 20;
        {
            let z = S2 {
                x: &x,
                y: &y
            };

            r = z.x;
        }
    }

    println!("Value is: {}", r);
}