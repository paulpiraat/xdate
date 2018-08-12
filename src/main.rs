extern crate chrono;
extern crate clap;
extern crate xcb;

use chrono::prelude::*;
use clap::{App, Arg};
use std::{thread, time};

fn main() {

    if !is_connected() {
        println!("Could not connect to X!");
        return;
    }

    let matches = App::new("xClock")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Use the given `printf` style format")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("snoop")
                .short("s")
                .long("snoop")
                .help("Active the snoop mode")
                .takes_value(false),
        )
        .get_matches();

    let format = matches.value_of("format").unwrap_or("");
    let snoop = matches.occurrences_of("snoop");

    if snoop == 1 {
        let wait_time = time::Duration::from_millis(1000);

        loop {
            if !is_connected() {
                break;
            }

            print_date(format);
            thread::sleep(wait_time);
        }
    } else {
        print_date(format);
    }
}

fn is_connected() -> bool {
    let entries = xcb::Connection::connect(None);
    let mut has_connection = false;
    for _entry in entries {
        has_connection = true;
        break;
    }
    return has_connection
}

fn print_date(format: &str) {
    let local: DateTime<Local> = Local::now();
    let dt = local.format("%a %e %b %T").to_string();
    println!("{}{}", format, dt);
}
