#[macro_use]
extern crate clap;

use clap::App;

use std::process;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let app_m = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    match app_m.subcommand() {
        ("get", Some(_sub_m)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("set", Some(_sub_m)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("rm", Some(_sub_m)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => {
            eprintln!("unimplemented 2");
            process::exit(1);
        }
    }
}
