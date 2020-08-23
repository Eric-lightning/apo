use clap::{App,AppSettings,Arg,ArgGroup,SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder) // args of help text sort by Defined.
        .version("0.1.0")
        .author("Yuuto Nakagawa <y.nakagawa@eric-lightning.info>")
        .about("termBased Apointment Manager to Myself with git-based sync and Simple editable.")
        .args_from_usage( // Display Settings
            "Next      -n --next      'List Only Next Apointment'
             Previous  -p --prsevious 'List Only Previous Apointment'
             PrintRAW  -1 --raw       'Print RAW'
             PrintCSV  -c --csv       'Print CSV style (No Quoted)'
             PrintJSON -j --json      'Print JSON Style (beta)'"
        )
        .arg(//year
			Arg::with_name("year")
                .help("set Year")
                .short("y")
                .long("year")
                .takes_value(true)
        )
        .arg(//month
			Arg::with_name("month")
                .help("set Month")
                .short("m")
                .long("month")
                .takes_value(true)
        )
        .arg(//day
			Arg::with_name("day")
                .help("set Day")
                .short("d")
                .long("day")
                .takes_value(true)
        )
        .arg(
			Arg::with_name("Future Date")
                .help("How many days from Today")
                .short(">")
                .takes_value(true)
        )
        .arg(
			Arg::with_name("Previous Date")
                .help("How many days ago")
                .short("<")
                .takes_value(true)
        )
        .subcommand(//add
			SubCommand::with_name("add")
                .about("Add Apointment")
                .arg(Arg::with_name("descriptions")
                    .help("hh:MM [Type][Important][Recururse] TEXT")
                    .last(true)
                )
        )
        .subcommand(//move
			SubCommand::with_name("mv")
            .about("Move to another time")
            .arg(Arg::with_name("from_time")
                    .help("hh:MM - move Targeted Time")
                    .conflicts_with("to_time")
                    .required(true)
            )
            .arg(Arg::with_name("to_time")
                    .help("hh:MM - Moved Time")
                    .required(true)
            )
        )
        .subcommand(//remove
	        SubCommand::with_name("rm")
                .about("Remove Apointment")
                .arg(Arg::with_name("time")
                    .help("hh:MM")
                    .required(true)
                )
        )
        .subcommand(//edit
		    SubCommand::with_name("edit")
                .about("edit Apointments by a day")
        )
        .subcommand(//modify
		   	SubCommand::with_name("modify")
                .about("modify apointment")
                .arg(Arg::with_name("time")
                    .help("hh:MM - edit time")
                    .required(true)
                )
                .args_from_usage(
                    "type      -t <TYPE>    'type 1:Schedule 2:Reminder 3:DeadLine'
                     important -i <Boolean> 'important: true or false'
                     recurse   -r <Boolean> 'recurse: true or false'"
                )
        )
        .subcommand(//note
		    SubCommand::with_name("note")
                .about("edit note by a Apointment")
                .arg(Arg::with_name("time")
                    .help("hh:MM")
                )
            )
        /*
        .group(ArgGroup::with_name("days")
            .args(&["today", "tom", "Future Date","Previous Date"])
        )*/
}
