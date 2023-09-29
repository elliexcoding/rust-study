use std::time;
use std::process;
use std::thread::{sleep};

fn main() {
    static mut SHUT_DOWN: bool = false;

    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("My pid is {}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!("{} seconds have passed", i);
    }

    // kill -SIGSTOP {PID}
}