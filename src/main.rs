extern crate chrono;
extern crate clap;

use std::{thread, time};
use chrono::prelude::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("xClock")
        .arg(Arg::with_name("format")
             .short("f")
             .long("format")
             .help("Use the given `printf` style format")
             .takes_value(true))
        .arg(Arg::with_name("snoop")
             .short("s")
             .long("snoop")
             .help("Active the snoop mode")
             .takes_value(false))
        .get_matches();

    let format = matches.value_of("format").unwrap_or("");
    let snoop = matches.occurrences_of("snoop");

    if snoop == 1 {
        let wait_time = time::Duration::from_millis(1000);

        loop {
            thread::sleep(wait_time);
            print_date(format);
        }
    } else {
        print_date(format);
    }
}

fn print_date(format: &str) {
    let local: DateTime<Local> = Local::now();
    let dt = local.format("%a %e %b %T").to_string();
    println!("{}{}", format, dt); 
}
