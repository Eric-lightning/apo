/*
 * apo/src/main.rs
 * Author: Eric-lightning <y.nakagawa at. eric-lightning.info
 *
 * TODO:
 * - Stdout with Colornize and Richstyle.
 *
 */

#[macro_use]
extern crate clap; // args analyze library.
mod cli; // imp cli module.
mod output;
use std::env;
use std::process;
use std::fs::File;
use std::string::String;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use chrono::{TimeZone,Duration,Timelike};

use chrono::prelude::{DateTime, Local, Datelike};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _matches = cli::build_cli().get_matches(); // Clap Args Analyzer.
    ////////////////////////////////////////////////////////////////////
    // GET ENV HOME
    let env_home = match env::var("HOME") {
        Ok(o)  => o,
        Err(_e) => {
            println!("Err: env::HOME is not set.");
            process::exit(1);
        }
    };
    ////////////////////////////////////////////////////////////////////
    // GET ENV APO DIRECTORY
    let keyword_apo_path = "APO_PATH";
    let default_apo_path = env_home + "/.apo";
    let defined_apo_path = match env::var(keyword_apo_path) {
        Ok(o)  => o,
        Err(_e) => {
            eprintln!("Info: No defined in env_var 'APO_PATH', will use `~/.apo`.");
            default_apo_path.to_string()
        }
    };
    ////////////////////////////////////////////////////////////////////
    // Define Date
    let mut date: DateTime<Local> = Local::now();
    let now_time = date.hour() * 60 + date.minute(); // TODO: use for --next and --previous
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
    // Define File Name
    let file_path = defined_apo_path.to_string() + "/"
        + &format!("{:04}",date.year() ).to_string() + "/"     // Year(>=CE) & Zero-Padding
        + &format!("{:02}",date.month()).to_string() + "/"     // Month(1-12)& Zero-Padding
        + &format!("{:02}",date.day()  ).to_string() + ".apo"; // Day(1-31)  & Zerp-Padding
    ////////////////////////////////////////////////////////////////////////
    // Read File
    let only_next = _matches.is_present("next");
    let only_prev = _matches.is_present("previous");
    let mut apo_datas = BTreeMap::new();
    for line_res in BufReader::new(File::open(file_path)?).lines() {
        // <<<<<<<<<<<<<<<<<<<< A line >>>>>>>>>>>>>>>>>>>>>>>>>>>
        let mut inner_map = HashMap::new();
        //
        let line  = line_res?;
        let cols: Vec<&str>  = line.split_whitespace().collect();
        let mut cols_control = cols.iter();
        //
        let time = cols_control.next().unwrap();
        inner_map.insert("time",time.to_string());
        //TODO: if include META
        let times: Vec<&str> = time.split(":").collect(); // TODO: META Date support! ALL,MRG,DAY,NGT,TASK
        //TODO: fi
        let time_h_num: i64 = times.get(0).unwrap().parse().unwrap();
        let time_m_num: i64 = times.get(1).unwrap().parse().unwrap();
        let time_sum_num: i64 = time_h_num * 60 + time_m_num;
        //
        let flags: Vec<char> = cols_control.next().unwrap().chars().collect();
        let flag_type = flags.get(0).unwrap().to_digit(10).unwrap();
        let flag_impt = flags.get(1).unwrap().to_digit(10).unwrap();
        let flag_recs = flags.get(2).unwrap().to_digit(10).unwrap();

        if flag_type == 0 { inner_map.insert("type"     ,"disable".to_string());}
        if flag_type == 1 { inner_map.insert("type"     ,"schedule".to_string());}
        if flag_type == 2 { inner_map.insert("type"     ,"reminder".to_string());}
        if flag_type == 3 { inner_map.insert("type"     ,"deadline".to_string());}
        if flag_impt == 0 { inner_map.insert("important","false".to_string());}
        if flag_impt == 1 { inner_map.insert("important","true".to_string());}
        if flag_recs == 0 { inner_map.insert("recurse"  ,"false".to_string());}
        if flag_recs == 1 { inner_map.insert("recurse"  ,"true".to_string());}
        ///////////////////////////////////////////////////////////
        // linked description
        let mut text_formating = String::new();
        let mut not_first_line     = false;
        //text_formating = String::new();
        let mut next           = cols_control.next();
        while None != next {
            if not_first_line {
                text_formating += " ";
            }
            else{
                not_first_line = true;
            }
            text_formating += next.unwrap();
            next = cols_control.next();
        }
        inner_map.insert("texts",text_formating);
        //
        apo_datas.insert(time_sum_num,inner_map);
        ///////////////////////
    }/*
    for (key, val) in apo_datas {
        println!("{} : {} -> {}", key, val.get("time").unwrap(), val.get("texts").unwrap());
    }*/
    if _matches.is_present("json") {
        println!("{}",output::to_json(&mut apo_datas));
    }
    if _matches.is_present("csv") {
        println!("{}",output::to_csv(&mut apo_datas));
    }
    if _matches.is_present("tsv") {
        println!("{}",output::to_tsv(&mut apo_datas));
    }
    Ok(())

}
fn prefix_one_char(arg: &str) -> char {
    return arg.chars().nth(0).unwrap();
}
fn skip_one_str_to_i64(arg: &str) -> i64 {
    return arg.chars().skip(1).collect::<String>().parse().unwrap();
}
