use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 

	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

	Ok(())
}


pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}


impl Config {
	pub fn new (mut args: env::Args) -> Result< Config, &'static str> {
		args.next();

		let query = match args.next(){
			Some(arg) => arg,
			None => return Err("No Query String"),
		};

		let filename = match args.next(){
			Some(arg) => arg,
			None => return Err("No Source File"),
		};

		let case_sensitive = match args.next(){
			Some(arg) => arg.contains("true"),
			None =>	env::var("CASE_INSENSITIVE").is_err(),
		};

		Ok(Config { query, filename, case_sensitive})

	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

	contents.lines()
	  .filter(|line| line.contains(query))
	  .collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

	let query = query.to_lowercase();

	contents.lines()
	  .filter(|line| line.to_lowercase().contains(&query))
	  .collect()

}


#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
			);
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
			);
	}
}