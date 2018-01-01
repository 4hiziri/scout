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
use std::fs;
use std::fs::File;
use std::io::{BufWriter, BufReader, Write, Read};
use std::fmt;

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

impl PathEntry {
    fn new(path: String, tags: Vec<String>) -> PathEntry {
        PathEntry {
            path: PathBuf::from(path),
            tags: tags.clone(),
        }
    }
}

impl fmt::Display for PathEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path = self.path.to_str().unwrap_or("PathEntry: fmt error");
        let mut tags_str: String = String::new();

        for tag in &self.tags {
            tags_str.push_str(&tag);
            tags_str.push_str(",");
        }

        tags_str.pop();

        write!(f, "{}: {}", path, tags_str)
    }
}

fn get_env_var() -> Option<PathBuf> {
    let env_key = "SCOUT_PATH";

    env::vars().find(|x| x.0 == env_key).map(|(_, val)| {
        PathBuf::from(val)
    })
}

fn get_store_path() -> Result<PathBuf, String> {
    let default = env::home_dir()
        .ok_or("$HOME doesn't exists!".to_owned())
        .and_then(|mut path| {
            path.push(".scout");
            Ok(path)
        });

    get_env_var().map_or(default, |path| Ok(path))
}

fn ensure_dir(path: &Path) -> Result<(), String> {
    if path.is_dir() && path.exists() {
        Ok(())
    } else {
        fs::create_dir_all(path).and_then(|_| Ok(())).map_err(|_| {
            format!("Failed create {}", path.to_str().unwrap_or("__missing__"))
        })
    }
}

fn read_path_entries(src: &Path) -> Result<Vec<PathEntry>, String> {
    let mut buf = String::new();

    // TODO: is this readable?
    File::open(src)
        .map_err(|_| "failed opening file".to_owned())
        .and_then(|f| Ok(BufReader::new(f))) // FIXME: early return?
        .and_then(|mut r| {
            r.read_to_string(&mut buf).map_err(|_| {
                "failed reading file".to_owned()
            })
        })
        .and_then(|_| {
            if buf.is_empty() {
                Ok(Vec::new())
            } else {
                json::decode::<Vec<PathEntry>>(&buf).map_err(|_| "parse error".to_owned())
            }
        })

}

fn write_path_entries(dst: &Path, entries: &Vec<PathEntry>) -> Result<(), String> {
    let payload = try!(json::encode(entries).map_err(|_| "parse error".to_owned()));

    File::create(dst)
        .map_err(|_| "failed creating file".to_owned())
        .and_then(|f| Ok(BufWriter::new(f)))
        .and_then(|mut w| {
            write!(w, "{}", payload).map_err(|_| "write error".to_owned())
        })
}

fn add_path(entry: PathEntry) -> Result<(), String> {
    // get worknig directory path
    let mut store_path = try!(get_store_path());
    try!(ensure_dir(store_path.as_path()));
    store_path.push("pathes.json");

    let store_path = store_path.as_path(); // remove mutability
    debug!("{:?}", &store_path);

    // if store file doesn't exist, create it here.
    if !store_path.exists() {
        try!(File::create(store_path).map_err(|_| {
            "failed creating file".to_owned()
        }));
    }

    let mut pathes = try!(read_path_entries(store_path));
    debug!("{:?}", pathes);

    try!(match pathes.iter().find(|e| e.path == entry.path) {
        Some(_) => Err("path exists!"),
        None => Ok(()),
    });

    pathes.push(entry);

    write_path_entries(store_path, &pathes)
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

        // TODO: setting lifetime and modified PathEntry
        let tags: Vec<String> = match matches.values_of("tags") {
            Some(tag_vec_str) => tag_vec_str.map(|x| x.to_owned()).collect(),
            None => Vec::new(),
        };

        let entry = PathEntry::new(input.to_string(), tags);

        // TODO: check whether file exists or not.
        println!("input is {}", input);
        println!("PathEntry: {}", entry.to_string());

        match add_path(entry) {
            Ok(_) => println!("Add path:"),
            Err(s) => println!("Failed!: {}", s),
        }
    }
}
