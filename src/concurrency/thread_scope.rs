#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_mutex() {
        let numbers = vec![1, 2, 3];

        // This pattern guarantees that none of the threads spawned in the scope can outlive the
        // scope
        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });
            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    }
}