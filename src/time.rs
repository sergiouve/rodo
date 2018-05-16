extern crate chrono;

use chrono::Local;

pub fn execute() {
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}
