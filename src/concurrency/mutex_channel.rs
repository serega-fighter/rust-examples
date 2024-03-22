#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

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
            self.item_ready.notify_one();
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
        let channel = MutexChanel::new();

        thread::scope(|s| {
            for _ in 0..3 {
                s.spawn(|| {
                    for i in 0..255 {
                        let msg = String::from("mes");
                        channel.send(msg);
                        println!("sent msg");
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
            for _ in 0..3 {
                s.spawn(|| {
                    for _ in 0..5000 {
                        println!("Received {:?}", channel.receive());
                    }
                });
            }

            thread::sleep(Duration::from_secs(20));
        });
    }
}