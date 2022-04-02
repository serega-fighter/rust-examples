struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn size(&self) -> usize {
        self.stack.len()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

fn main() {
    let mut st = Stack {stack: Vec::new() };

    st.push(String::from("ABC"));
    st.push(String::from("DEF"));
    st.push(String::from("CDA"));

    println!("Stack has {} elements", st.size());
}