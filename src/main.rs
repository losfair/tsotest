use std::ptr::{read_volatile, write_volatile};
use std::sync::mpsc::{SyncSender, Receiver, channel, sync_channel};

static mut V0: u64 = 0;
static mut V1: u64 = 0;
static mut V2: u64 = 0;

fn th1(ch: SyncSender<()>, begin: Receiver<()>) {
    enable_tso_thread();
    loop {
        begin.recv().unwrap();
        unsafe {
            write_volatile(&mut V0, 1);
            write_volatile(&mut V1, 1);
        }
        ch.send(()).unwrap();
    }
}

fn th2(ch: SyncSender<()>, begin: Receiver<()>) {
    enable_tso_thread();
    loop {
        begin.recv().unwrap();
        unsafe {
            while read_volatile(&V1) == 0 {}
            if read_volatile(&V0) != 0 {
                write_volatile(&mut V2, 1);
            }
        }
        ch.send(()).unwrap();
    }
}

fn enable_tso_thread() {
    #[cfg(feature = "apple-tso")]
    {
        use sysctl::Sysctl;
        sysctl::Ctl::new("kern.tso_enable").unwrap().set_value_string("1").unwrap();
    }
}

fn main() {
    enable_tso_thread();
    let (begin1_tx, begin1_rx) = channel();
    let (begin2_tx, begin2_rx) = channel();
    let (tx1, rx1) = sync_channel(0);
    let (tx2, rx2) = sync_channel(0);
    std::thread::spawn(|| th1(tx1, begin1_rx));
    std::thread::spawn(|| th2(tx2, begin2_rx));
    for i in 0..10000000usize {
        begin2_tx.send(()).unwrap();
        begin1_tx.send(()).unwrap();
        rx2.recv().unwrap();
        rx1.recv().unwrap();
        unsafe {
            if read_volatile(&V2) != 1 {
                panic!("TSO test failed at iteration {}", i);
            }
            write_volatile(&mut V0, 0);
            write_volatile(&mut V1, 0);
            write_volatile(&mut V2, 0);
        }
        if i % 500000 == 0 {
            println!("progress: {}", i);
        }
    }
}
