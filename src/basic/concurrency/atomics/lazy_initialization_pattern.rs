use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn calculate_x() -> u64 {
    2 * 5
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

pub fn main() {
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Calculated: {}", get_x());
        });
        scope.spawn(|| {
            println!("Calculated: {}", get_x());
        });
    });
}