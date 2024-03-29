struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    fn convert(mut self) -> Vec<T> {
        let mut out: Vec<T> = Vec::new();
        while !self.is_empty() {
            out.push(self.dequeue());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::queue::Queue;

    #[test]
    fn test_get() {
        let mut queue: Queue<String> = Queue::new();
        queue.enqueue(String::from("ABC"));
        queue.enqueue(String::from("DEF"));
        queue.enqueue(String::from("CGA"));

        let item = queue.dequeue();
        assert_eq!(item, String::from("ABC"));

        let item2: Option<&String> = queue.peek();
        let str_ref: &String = item2.unwrap();
        let val_casted: &str = str_ref;
        println!("Item 2: {}", val_casted);

        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_convert_ownership() {
        let mut queue: Queue<String> = Queue::new();
        queue.enqueue(String::from("ABC"));
        queue.enqueue(String::from("DEF"));
        queue.enqueue(String::from("CGA"));

        let vec = queue.convert();

        assert_eq!(3, vec.len());
    }
}