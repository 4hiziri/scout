#[macro_use]
extern crate clap;
extern crate rustc_serialize;
extern crate scout;
use scout::testmod;
use clap::App;
use rustc_serialize::json;
use std::path::PathBuf;
use std::env;
use std::fs::File;

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

fn get_store_path() -> PathBuf {
    match get_env_var() {
        Some(path) => path,
        None => {
            let mut tmp = env::home_dir().unwrap();
            tmp.push(".scout");
            tmp
        }
    }
}

fn add_path(file_path: String, tags: Vec<String>) {
    let mut path_entries: Vec<PathEntry> = Vec::new();
    let mut store_path = get_store_path();
    store_path.push("pathes.json");
    let store_path = store_path;
    println!("{:?}", &store_path);
    // FIX: if .scout doesn't exist, create_dirx
    let mut pathes_file = if !store_path.exists() {
        // TODO: error handling
        File::create(store_path).unwrap()
    } else {
        File::open(store_path).unwrap()
    };
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

        // TODO: check whether file exists or not.
        println!("input is {}", input);

        // TODO: tag parser
        if matches.is_present("tags") {
            let tags = matches.value_of("tags").unwrap_or("nothing");
            println!("tags is {}", tags);
        }

        add_path(input.to_string(), Vec::new());
    }
}
