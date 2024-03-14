use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;

#[derive(Debug)]
struct Data {
    name: String,
    age: u32,
}

fn generate_data() -> Data {
    return Data {name: String::from("Serega"), age: 29}
}

fn get_data() -> &'static Data {

    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) }); // drop explicitly
            p = e;
        }
    }

    unsafe { &*p }
}

pub fn main() {
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Calculated: {:?}", generate_data());
        });
        scope.spawn(|| {
            println!("Calculated: {:?}", generate_data());
        });
    });
}