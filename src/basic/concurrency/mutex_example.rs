use std::sync::Mutex;
use std::thread;

fn main() {
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
