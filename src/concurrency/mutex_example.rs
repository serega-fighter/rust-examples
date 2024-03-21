#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::thread;

    #[test]
    fn test_mutex() {
        let n = Mutex::new(0);
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut guard = n.lock().unwrap();
                    for _ in 0..5000 {
                        *guard += 1;
                    }
                });
            }
        });
        println!("The result is {:?}", n.into_inner().unwrap())
    }
}