extern crate clap;
extern crate chrono;

use clap::{Arg, App};
use chrono::Local;

fn main() {
    let matches = App::new("rodo")
        .version("0.0.1")
        .about("Personal CLI assistant")
        .author("Sergio Uve")
        .arg(Arg::with_name("module")
             .help("Module to load"))
        .get_matches();

    let module = matches.value_of("module").unwrap_or("hello");

    match module {
        "time" => time(),
        _ => hello(),
    }
}

fn hello() {
    println!("Hi, my name is rodo");
}

fn time() {
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}
