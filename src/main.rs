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
    #[allow(dead_code)]
    fn from_string(path: String, tags: Vec<String>) -> PathEntry {
        PathEntry::new(PathBuf::from(path), tags)
    }

    fn new(path: PathBuf, tags: Vec<String>) -> PathEntry {
        PathEntry {
            path: path,
            tags: tags.clone(),
        }
    }
}

impl PartialEq for PathEntry {
    fn eq(&self, other: &PathEntry) -> bool {
        self.path == other.path && self.tags == other.tags
    }
}

impl Eq for PathEntry {}

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

fn get_store_file_path() -> Result<PathBuf, String> {
    let mut store_path = try!(get_store_path());
    try!(ensure_dir(store_path.as_path()));
    store_path.push("pathes.json");

    Ok(store_path)
}

fn rel_abs(path: &PathBuf) -> Result<PathBuf, String> {
    env::current_dir()
        .and_then(|mut abs_path| {
            abs_path.push(path);
            Ok(abs_path)
        })
        .map_err(|_| "can't get currend directory".to_owned())
}

fn abs_if_exists(path: PathBuf) -> PathBuf {
    match rel_abs(&path) {
        Ok(abspath) => if abspath.exists() { abspath } else { path },
        Err(_) => path,
    }
}

fn add_path(entry: PathEntry) -> Result<(), String> {
    let store_path = try!(get_store_file_path());
    debug!("{:?}", &store_path);

    // if store file doesn't exist, create it here.
    if !store_path.exists() {
        try!(File::create(&store_path).map_err(|_| {
            "failed creating file".to_owned()
        }));
    }

    let mut pathes: Vec<PathEntry> = try!(read_path_entries(&store_path));
    debug!("add_path: read_path_enties -> {:?}", pathes);

    if !pathes.iter().any(|e| e == &entry) {
        pathes.push(entry);
        write_path_entries(&store_path, &pathes)
    } else {
        Err("Path exists!".to_owned())
    }
}

fn list_path() -> Result<(), String> {
    let store_file = try!(get_store_file_path());

    if store_file.exists() {
        let entries = try!(read_path_entries(&store_file));

        for e in entries {
            println!("{}", e);
        }

        Ok(())
    } else {
        let file_str = match store_file.to_str() {
            Some(s) => s,
            None => "None",
        };

        Err(format!("{} doesn't exists!", file_str))
    }
}

fn remove_path(path: PathBuf) -> Result<(), String> {
    let store_file = try!(get_store_file_path());

    if store_file.exists() {
        let pathes: Vec<_> = try!(read_path_entries(&store_file));

        write_path_entries(
            &store_file,
            &pathes.into_iter().filter(|x| x.path != path).collect(),
        )
    } else {
        Err("File doesn't exists".to_owned())
    }
}

fn main() {
    env_logger::init().unwrap();

    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!());

    match app.get_matches().subcommand() {
        ("add", Some(matches)) => {
            let path = abs_if_exists(PathBuf::from(matches.value_of("PATH").unwrap()));

            // TODO: setting lifetime and modified PathEntry
            let tags: Vec<String> = match matches.values_of("tags") {
                Some(tag_vec_str) => tag_vec_str.map(|x| x.to_owned()).collect(),
                None => Vec::new(),
            };

            let entry = PathEntry::new(path, tags);

            match add_path(entry) {
                Ok(_) => debug!("Success"),
                Err(s) => println!("Error add: {}", s),
            }
        }
        ("find", Some(_matches)) => {
            println!("Not impl!");
        }
        ("remove", Some(matches)) => {
            let path = abs_if_exists(PathBuf::from(matches.value_of("PATH").unwrap()));

            remove_path(path).unwrap();
        }
        ("list", Some(_matches)) => {
            match list_path() {
                Ok(_) => debug!("Success ls"),
                Err(s) => println!("Failed list path: {}", s),
            }
        }
        (&_, _) => {
            println!("command missing!");
        }
    }
}
