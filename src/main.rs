#[macro_use]
#[allow(unused_imports)]
extern crate clap;
extern crate rustc_serialize;
use clap::App;
// use rustc_serialize::json;
use std::path::{PathBuf, Path};
use std::env;
use std::fs::File;
use std::io;

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

fn ensure_dir(path: &Path) -> Result<(), String> {
    use std::fs;

    if path.exists() {
        Ok(())
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Failed create {}",
                path.to_str().unwrap_or("__missing__")
            )),
        }
    }
}

fn add_path(_file_path: String, _tags: Vec<String>) {
    let _path_entries: Vec<PathEntry> = Vec::new();
    let mut store_path = get_store_path();
    store_path.push("pathes.json");
    let store_path = store_path;
    println!("{:?}", &store_path);
    // FIX: if .scout doesn't exist, create_dir
    let _pathes_file = if !store_path.exists() {
        // TODO: error handling
        File::create(store_path).unwrap()
    } else {
        File::open(store_path).unwrap()
    };
}

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
