#[macro_use]
extern crate clap; // args analyze library.
mod cli; // imp cli module.

use std::process;

fn main() {
    let _matches = cli::build_cli().get_matches(); // Clap Args Analyzer.
    //println!("Invalid Argments!");
    //process::exit(1);
    // DateObj
    let _toyear  = 2020;
    let _tomonth = 8;
    let _today   = 25;

    let _year  = if let Some(o) = _matches.value_of("year")  { dateDetector(o,_toyear) } else { _toyear  };
    let _month = if let Some(o) = _matches.value_of("month") { dateDetector(o,_tomonth)} else { _tomonth };
    let _day   = if let Some(o) = _matches.value_of("day")   { dateDetector(o,_today)  } else { _today   };

    println!("TEST DATE DEF: {0} {1} {2}",_year,_month,_day);

    // ReDef Date
}

fn dateDetector(arg: &str, nowVal: i64) -> i64 {
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
