use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn do_some_work() {

}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            do_some_work();
        }
    });

    thread::sleep(Duration::from_secs(1));

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}