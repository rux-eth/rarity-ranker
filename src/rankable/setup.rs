use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::{collections::HashMap, env, error::Error, fmt::Display, fs, io, path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub history: HashMap<String, HashMap<String, String>>,
    pub settings: Settings,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub precision: f64,
    pub trait_path: path::PathBuf,
}

pub fn get_config() -> Config {
    let c = Config {
        history: get_history(),
        settings: get_settings(),
    };
    save_progress(&c);
    c
}
fn get_history() -> HashMap<String, HashMap<String, String>> {
    let mut cursor: path::PathBuf = config_dir();
    cursor.push("cache.json");
    if cursor.is_file() {
        let file = fs::File::open(&cursor).expect("Error reading settings");
        let reader = io::BufReader::new(file);
        match from_reader(reader) {
            Ok(res) => {
                println!("Saved progress file found. Would you like to continue progress?");
                let choice = Select::with_theme(&ColorfulTheme::default())
                    .item("Continue progress")
                    .item("Start over")
                    .default(0)
                    .interact()
                    .expect("Could not read input");
                match choice {
                    0 => res,
                    _ => {
                        println!("Are you sure you want to start over? All previous progress will be lost and overwritten");
                        let choice = Select::with_theme(&ColorfulTheme::default())
                            .item("Do not start over")
                            .item("Start over")
                            .default(0)
                            .interact()
                            .expect("Could not read input");
                        match choice {
                            0 => get_history(),
                            _ => HashMap::new(),
                        }
                    }
                }
            }
            Err(_) => {
                println!("No saved progress found. Starting new progress");
                HashMap::new()
            }
        }
    } else {
        println!("No saved progress found. Starting new progress");

        HashMap::new()
    }
}
fn get_settings() -> Settings {
    fn new_settings() -> Settings {
        let mut cursor: path::PathBuf = config_dir();
        cursor.pop();
        cursor.push("in");
        Settings {
            precision: 0.98,
            trait_path: cursor,
        }
    }
    let mut cursor: path::PathBuf = config_dir();
    cursor.push("settings.json");
    if cursor.is_file() {
        let file = fs::File::open(&cursor).expect("Error reading settings");
        let reader = io::BufReader::new(file);
        match from_reader(reader) {
            Ok(res) => {
                println!("Saved settings file found. Use previous settings?");
                let choice = Select::with_theme(&ColorfulTheme::default())
                    .item("Use saved settings")
                    .item("Make new settings")
                    .default(0)
                    .interact()
                    .expect("Could not read input");
                if choice == 0 {
                    res
                } else {
                    new_settings()
                }
            }
            Err(_) => {
                println!("No saved settings file found.");
                new_settings()
            }
        }
    } else {
        println!("No saved settings file found.");

        new_settings()
    }
}

fn config_dir() -> path::PathBuf {
    let mut cursor: path::PathBuf;
    if let Ok(dir) = env::current_exe() {
        cursor = match dir.parent() {
            Some(p) => p.to_path_buf(),
            None => panic!("Invalid location for executable. Please read README for details."),
        };
        cursor.push("config");
        if !cursor.is_dir() {
            panic!("No config directory found. Please read README");
        }
        return cursor;
    } else {
        panic!("Could not determine exe directory")
    }
}
pub fn save_progress(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut cursor: path::PathBuf = config_dir();
    cursor.push("settings.json");
    to_writer_pretty(&fs::File::create(&cursor)?, &config.settings)?;
    cursor.pop();
    cursor.push("cache.json");
    to_writer_pretty(&fs::File::create(&cursor)?, &config.history)?;
    Ok(())
}
