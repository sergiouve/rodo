extern crate clap;

use clap::{Arg, App};

mod hello;
mod time;

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
        "time" => time::execute(),
        _ => hello::execute()
    }
}
