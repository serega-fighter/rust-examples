use std::sync::atomic::{AtomicBool, fence};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
use std::time::Duration;

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn compute_value(id: i32) -> u64 {
    let mut res: u64 = 5;
    for x in 0..54440 {
        res += x as u64 * id as u64;
    }
    return res
}

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = compute_value(i);
            unsafe { DATA[i as usize] = data };
            READY[i as usize].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {

        /**
        Instead of using 10 acquire-load operations to read the booleans, the main thread
        uses relaxed operations and a single acquire fence. It executes the fence before
        reading the data, but only if there is data to be read.
         */
        fence(Acquire);

        for i in 0..10 {
            if ready[i as usize] {
                println!("data{i} = {}", unsafe { DATA[i as usize] });
            }
        }
    }
}

