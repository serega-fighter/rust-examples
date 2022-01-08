
struct Point<T, U> {
    x: T,
    y: U,
}

pub trait Printable {
    fn print(&self) -> String {
        String::from("Default value")
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T, U> Printable for Point<T, U> {
    fn print(&self) -> String {
        String::from("ABC")
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("printed = {}", p3.print());

    let ve = vec![1, 4, 2, 0];
    println!("Largest of vec {:?} is {}", ve, largest(&ve))
}