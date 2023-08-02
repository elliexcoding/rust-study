use std::{thread, time};

fn main () {
    let start = time::Instant::now();

    let handler = thread::span(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });

    handlerjoin().unwrap();

    let elapsed = time::Instant::now();

    println!("Elapsed: {:?}", elapsed.duration_since(start))
}