#[macro_use]
extern crate clap;
extern crate rustc_serialize;
extern crate env_logger;
#[macro_use]
extern crate log;
use clap::App;
use rustc_serialize::json;
use std::path::{PathBuf, Path};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

/// scout - doc management tool
/// # goal
/// manage file by tagging
/// search doc by query and tag
///
/// # data storing
/// will store data to plain txt as json, xml or so on.
/// tag is stored to separated file.
/// revese reference
#[derive(Debug, RustcDecodable, RustcEncodable)]
struct PathEntry {
    path: PathBuf,
    tags: Vec<String>,
}

fn get_env_var() -> Option<PathBuf> {
    let env_key: String = "SCOUT_PATH".to_owned();

    match env::vars().find(|x| x.0 == "SCOUT_PATH") {
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

    if path.is_dir() && path.exists() {
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

fn read_path_entries(path: &Path) -> Result<Vec<PathEntry>, String> {
    let mut buf = String::new();

    // TODO: is this readable?
    File::open(path)
        .map_err(|_| "failed opening file".to_owned())
        .and_then(|f| Ok(BufReader::new(f)))
        .and_then(|mut r| {
            r.read_to_string(&mut buf).map_err(|_| {
                "failed reading file".to_owned()
            })
        })
        .and_then(|_| {
            json::decode::<Vec<PathEntry>>(&buf).map_err(|_| "parse error".to_owned())
        })

}

fn add_path(_file_path: String, _tags: Vec<String>) -> Result<(), String> {
    let mut store_path = get_store_path();
    ensure_dir(store_path.as_path()).unwrap(); // TODO: error check
    store_path.push("pathes.json");

    let store_path = store_path.as_path();
    debug!("{:?}", &store_path);

    // if store file doesn't exist, create it here.
    if !store_path.exists() {
        File::create(store_path).unwrap();
    }

    let pathes = read_path_entries(store_path).unwrap();
    debug!("{:?}", pathes);


    Err("Not implemented yet!".to_owned())
}

fn main() {
    env_logger::init().unwrap();

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

        add_path(input.to_string(), Vec::new()).unwrap();
    }
}
