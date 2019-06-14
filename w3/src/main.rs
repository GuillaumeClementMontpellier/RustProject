use std::env;
use std::process;

use w3::Config;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = w3::run(config) {		
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
	
}
