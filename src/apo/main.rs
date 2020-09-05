/*
 * apo/src/main.rs
 * Author: Eric-lightning <y.nakagawa at. eric-lightning.info
 *
 * TODO:
 * - Parse Loaded Files
 * - OutputOpt e.g. JSON,CSV
 * = Stdout with Colornize and Richstyle.
 *
 */

#[macro_use]
extern crate clap; // args analyze library.
mod cli; // imp cli module.

use std::env;
use std::process;
use std::fs::File;
use std::string::String;
use std::io::{self, BufRead, BufReader, Read};
use chrono::{TimeZone, Weekday, ParseResult,Duration};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};
use chrono::offset::FixedOffset;

fn main() -> Result<(), Box<std::error::Error>> {
    let _matches = cli::build_cli().get_matches(); // Clap Args Analyzer.
    ////////////////////////////////////////////////////////////////////
    // GET ENV HOME
    let env_home = match env::var("HOME") {
        Ok(o)  => o,
        Err(e) => {
            println!("Err: env::HOME is not set.");
            process::exit(1);
        }
    };
    ////////////////////////////////////////////////////////////////////
    // GET ENV APO
    let keyword_apo_path = "APO_PATH";
    let default_apo_path = env_home + "/.apo";
    let defined_apo_path = match env::var(keyword_apo_path) {
        Ok(o)  => o,
        Err(e) => {
            println!("Info: No defined in env_var 'APO_PATH', will use `~/.apo`.");
            default_apo_path.to_string()
        }
    };
    ////////////////////////////////////////////////////////////////////
    // Define Date
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
    ////////////////////////////////////////////////////////////////////////
    // Define File
    let file_path = default_apo_path.to_string() + "/"
        + &format!("{:04}",date.year() ).to_string() + "/"     // Year(>=CE) & Zero-Padding
        + &format!("{:02}",date.month()).to_string() + "/"     // Month(1-12)& Zero-Padding
        + &format!("{:02}",date.day()  ).to_string() + ".apo"; // Day(1-31)  & Zerp-Padding
    ////////////////////////////////////////////////////////////////////////
    // Read
    for line_res in BufReader::new(File::open(file_path)?).lines() {
        let line  = line_res?;
        println!("{}",line);
        let cols: Vec<&str>  = line.split_whitespace().collect();
        let mut cols_control = cols.iter();
        let time  = cols_control.next().unwrap();
        let flags = cols_control.next().unwrap();
        let mut texts = cols_control.next().unwrap().to_string();
        let mut next  = cols_control.next();
        while None != next {
            texts.push(' ');
            texts.push_str(next.unwrap());
            next = cols_control.next();
        }
        println!("time:  {}", time.to_string());
        println!("flags: {}",flags);
        println!("texts: {}",texts);

        //let = white_space_array.next();
    }
    Ok(())



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
