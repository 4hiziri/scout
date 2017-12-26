#[macro_use]
extern crate clap;
use clap::App;
use std::path;
use std::env;

/// scout - doc management tool
/// # goal
/// manage file by tagging
/// search doc by query and tag

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let input = matches.value_of("PATH").unwrap();

        println!("input is {}", input);

        if matches.is_present("tags") {
            let tags = matches.value_of("tags").unwrap_or("nothing");
            println!("tags is {}", tags);
        }
    }
}
