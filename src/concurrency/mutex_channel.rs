#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Condvar, Mutex};

    pub struct MutexChanel<T> {
        queue: Mutex<VecDeque<T>>,
        item_ready: Condvar
    }

    impl<T> MutexChanel<T> {
        pub fn new() -> Self {
            Self {
                queue: Mutex::new(VecDeque::new()),
                item_ready: Condvar::new()
            }
        }

        pub fn send(&self, message: T) {
            self.queue.lock().unwrap().push_back(message);
            self.item_ready.notify_all();
        }

        pub fn receive(&self) -> T {
            let mut b = self.queue.lock().unwrap();
            loop {
                if let Some(message) = b.pop_front() {
                    return message;
                }
                b = self.item_ready.wait(b).unwrap();
            }
        }
    }

    #[test]
    fn test_mutex_channel() {
        let mut channel = MutexChanel::new();
        channel.send(String::from("AABSASD"));

        println!("{:?}", channel.receive());
    }
}