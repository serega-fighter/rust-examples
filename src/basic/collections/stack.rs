struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
}

fn main() {
    let s = Stack {stack: Vec::new() };

}