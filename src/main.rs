use chrono::{Datelike, Local};
use rust_experiments::calendar_functions;

fn main() {
    let today = Local::now();
    println!("Hello in {0} day of the year.", calendar_functions::day_of_year(today.year() as u32, today.month(), today.day()));
}
