use crate::day_at_work::DayAtWork;
use crate::responsibility::Responsibility;
use chrono::naive::NaiveDate;
use serde_yaml::from_reader;
use std::env::args;
use std::fs::File;

mod activity;
mod day_at_work;
mod jitter;
mod responsibility;

fn main() {
    const WORK_HOURS: f64 = 8.0;
    const RESOLUTION: f64 = 0.25;

    const DATE_FORMAT: &str = "%Y-%m-%d";

    let arguments: Vec<String> = args().collect();

    let yaml = File::open(&arguments[1]).unwrap();

    let responsibilities: Vec<Responsibility> = from_reader(&yaml).unwrap();

    let date = NaiveDate::parse_from_str(&arguments[2], DATE_FORMAT).unwrap();

    let day = DayAtWork::new(date, WORK_HOURS, RESOLUTION, &responsibilities);

    println!("{:#?}", day);
}
