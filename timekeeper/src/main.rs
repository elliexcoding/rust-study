use chrono::Local;
use chrono::{DateTime};
use chrono::{Local};


struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}


fn main() {
    let now = Local::now();
    println!("Hello, world! The time is {}.", now)
}
