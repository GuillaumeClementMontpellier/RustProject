// This example demonstrates clap's building from YAML style of creating arguments which is far
// more clean, but takes a very small performance hit compared to the other two methods.
#[macro_use]
extern crate clap;

#[macro_use]
extern crate dotenv_codegen;

use std::env;

use clap::App;

use std::process;

fn main() {

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("verbose") {
    	0 => println!("No verbose info"),
    	1 => println!("Some verbose info"),
    	2 => println!("Tons of verbose info"),
    	3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
    	if matches.is_present("debug") {
    		println!("Printing debug info...");
    	} else {
    		println!("Printing normally...");
    	}
    }

    println!("{}", dotenv!("PORT"));

    let key = "HOME";

    match env::var_os(key) {
    	Some(val) => println!("{}: {:?}", key, val),
    	None => println!("{} is not defined in the environment.", key)
    }

    process::exit(0); // != 0 veut dire error, seulement visible avec cargo run

    // more program logic goes here...
}