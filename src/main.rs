extern crate chrono;

use std::{thread, time};
use chrono::prelude::*;

fn main() {
    let wait_time = time::Duration::from_millis(1000);

    loop {
        thread::sleep(wait_time);
        print_date();
    }
}

fn print_date() {
    let local: DateTime<Local> = Local::now(); // 2018-08-10 17:30:04.2312312312 +02:00
    let dt = local.format("%a %e %b %T").to_string();
    println!("{}", dt); 
}
