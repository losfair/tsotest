use std::sync::atomic::{AtomicBool, Ordering};

static B0: AtomicBool = AtomicBool::new(false);
static B1: AtomicBool = AtomicBool::new(false);
static B2: AtomicBool = AtomicBool::new(false);

fn worker() {
    loop {
        B0.store(true, Ordering::Relaxed);
        B1.store(true, Ordering::Relaxed);
        B2.store(true, Ordering::Relaxed);
        while B2.load(Ordering::Relaxed) == true {}
        B1.store(false, Ordering::Relaxed);
        B0.store(false, Ordering::Relaxed);
    }
}

fn main() {
    std::thread::spawn(worker);
    for i in 0..1000000000usize {
        while B1.load(Ordering::Relaxed) == false {}
        if B0.load(Ordering::Relaxed) != true {
            panic!("TSO check failed at iteration {}", i);
        }
        B2.store(false, Ordering::Relaxed);
    }
}
