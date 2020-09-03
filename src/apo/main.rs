#[macro_use]
extern crate clap; // args analyze library.
mod cli; // imp cli module.

use std::env;

use chrono::{TimeZone, Weekday, ParseResult,Duration};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};
use chrono::offset::FixedOffset;

fn main() {
    let _matches = cli::build_cli().get_matches(); // Clap Args Analyzer.
    //println!("Invalid Argments!");
    //process::exit(1);
    let keyword_apo_path = "APO_PATH";
    let default_apo_path = "~/.apo";
    let defined_apo_path = match std::env::var(keyword_apo_path) {
        Ok(o)  => o,
        Err(e) => {
            println!("Info: No defined in env_var 'APO_PATH', will use `~/.apo`.");
            default_apo_path.to_string()
        }
    };
    //println!("def_var: {}",defined_apo_path); // DEBUG: Print def_apo_path

    // 4. DATE DEF
    // 5. LOAD File
    // 6. Parse File
    // 7. OutputOpts
    // 8. STDOUT

    let mut date: DateTime<Local> = Local::now();

    if let Some(o) = _matches.value_of("day") {
        if prefix_one_char(o) == '+' { date = date + Duration::days(skip_one_str_to_i64(o)); }
        else if prefix_one_char(o) == '-' { date = date + Duration::days(0 - skip_one_str_to_i64(o)); }
        else { date = Local.ymd(date.year(), date.month(), o.parse().unwrap()).and_hms(0,00,00); }
    };
    if let Some(o) = _matches.value_of("month") {
        if prefix_one_char(o) == '+' { date = date + Duration::weeks(4 * skip_one_str_to_i64(o)); }
        else if prefix_one_char(o) == '-' { date = date + Duration::weeks(4 * (0 - skip_one_str_to_i64(o))); }
        else { date = Local.ymd(date.year(),o.parse().unwrap(), date.day()).and_hms(0,00,00); }
    };
    if let Some(o) = _matches.value_of("year")  {
        if prefix_one_char(o) == '+' { date = date + Duration::weeks(12 * 4 * skip_one_str_to_i64(o)); }
        else if prefix_one_char(o) == '-' { date = date - Duration::weeks(12 * 4 * skip_one_str_to_i64(o)); }
    };
    if let Some(o) = _matches.value_of("week") {
        if prefix_one_char(o) == '+' { date = date + Duration::weeks(skip_one_str_to_i64(o)); }
        else if prefix_one_char(o) == '-' { date = date + Duration::weeks(0 - skip_one_str_to_i64(o)); }
        else { date = Local.ymd(o.parse().unwrap(), date.month(), date.day()).and_hms(0,00,00); }
    };

    /////////////////////
    println!("TEST DATE DEF: {0} {1} {2}",date.year(),date.month(),date.day()); //DEBUG


}
fn prefix_one_char(arg: &str) -> char {
    return arg.chars().nth(0).unwrap();
}
fn skip_one_str_to_i64(arg: &str) -> i64 {
    return arg.chars().skip(1).collect::<String>().parse().unwrap();
}
/*
fn date_detector(arg: &str, nowVal: i64) -> i64 {
    if arg.chars().nth(0).unwrap() == '+' {
        let argVal: i64 = arg.chars().skip(1).collect::<String>().parse().unwrap();
        return nowVal + argVal;
    }
    if arg.chars().nth(0).unwrap() == '-' {
        let argVal: i64 = arg.chars().skip(1).collect::<String>().parse().unwrap();
        return nowVal - argVal;
    }
    return arg.parse().unwrap();
}
*/
