use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn generate_id() -> u32 {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    let mut current = NEXT.load(Relaxed);
    loop {
        let result = NEXT.compare_exchange_weak(current, current + 1, Relaxed, Relaxed);

        match result {
            Ok(_) => {
                return current;
            },
            Err(actual) => {
                current = actual
            }
        }
    }
}

fn main() {

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..1000000 {
                generate_id();
            }
        });
        s.spawn(|| {
            for _ in 0..1000000 {
                generate_id();
            }
        });
    })
}