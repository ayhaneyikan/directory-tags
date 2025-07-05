mod args;

use args::Cli;
use clap::Parser;
use serde::{Deserialize, Serialize};

use std::{
    cmp,
    collections::HashMap,
    env, fs, io,
    path::{Path, PathBuf},
};

const SAVE_FILE_NAME: &str = ".directory-tags";

/// Defines directory mapping data saved to file
#[derive(Serialize, Deserialize, Debug)]
struct DirMap {
    /// Path to the save file
    path: PathBuf,
    map: HashMap<String, PathBuf>,
}

impl DirMap {
    /// Attempts to load the directory mapping data from file.
    /// If unable to, initializes a new dir mapping data file.
    fn load_or_init() -> io::Result<Self> {
        // create save file path
        let mut path = env::home_dir().expect("Unable to get user home directory");
        path.push(SAVE_FILE_NAME);

        // attempt to load data from the file
        let map = if path.is_file() {
            let data = fs::read_to_string(&path)?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };

        //Hey baby i really really love you. You're a coding beast <3
        Ok(Self { path, map })
    }

    /// Saves map into file
    fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&self.map)?;
        fs::write(&self.path, data)
    }

    /// Add new tag path mapping
    // TODO consider at clean way of taking impl Into<String> to smooth out api
    fn add_tag<P: AsRef<Path>>(&mut self, tag: String, path: P) {
        self.map.insert(tag.clone(), path.as_ref().to_path_buf());
        println!("Added tag: {} -> {}", tag, path.as_ref().display());
    }

    fn get_path(&self, tag: &str) -> Option<&PathBuf> {
        self.map.get(tag)
    }

    /// Print out all directory mappings
    fn list_all(&self) {
        // compute longest tag name to use for print formatting
        let max_tag_len = &self.map.keys().fold(0, |acc, e| cmp::max(acc, e.len()));
        // print each mapping
        for (tag, path) in &self.map {
            // print using whitespace padding up to the largest tag name
            println!("{:<max_tag_len$} -> {}", tag, path.display());
        }
    }

    /// Print out all available tags
    fn list_tags(&self) {
        for tag in self.map.keys() {
            println!("{}", tag);
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    // TODO add custom validation for use of --path without --add

    let mut dir_map = DirMap::load_or_init()?;

    // execute based on given args
    if let Some(tag) = args.add {
        // add new tag to the map
        match args.path {
            Some(path) => dir_map.add_tag(tag, path),
            None => dir_map.add_tag(tag, env::current_dir()?),
        };
        dir_map.save()?;
    } else if args.list {
        dir_map.list_all();
    } else if args.list_tags {
        dir_map.list_tags();
    } else if let Some(tag) = args.tag {
        match dir_map.get_path(&tag) {
            Some(path) => {
                // print to stdout; shell wrapper handles `cd`
                println!("{}", path.display());
                std::process::exit(2);
            }
            None => {
                eprintln!("Error: tag '{tag}' not found");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
