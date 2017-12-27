#[macro_use]
extern crate clap;
extern crate rustc_serialize;
extern crate scout;
use scout::testmod;
use clap::App;
use rustc_serialize::json;
use std::path::PathBuf;
use std::env;

/// scout - doc management tool
/// # goal
/// manage file by tagging
/// search doc by query and tag
///
/// # data storing
/// will store data to plain txt as json, xml or so on.
/// tag is stored to separated file.
/// revese reference

struct PathEntry {
    path: PathBuf,
    tags: Vec<String>,
}

fn get_env_var() -> Option<PathBuf> {
    let env_vars = env::vars();

    match env_vars.filter(|x| x.1 == "SCOUT_PATH").next() {
        Some((_, var)) => Some(PathBuf::from(var)),
        None => None,
    }
}

fn add_path(path: String, tags: Vec<String>) {
    let path = get_env_var().unwrap_or(env::home_dir().unwrap());
}

fn main() {
    testmod::test();

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
