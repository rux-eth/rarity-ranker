mod rating_algo;
pub mod setup;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;
use serde_json::{from_reader, to_writer_pretty};
use std::error::Error;
use std::{collections::HashMap, env, fs, path};

use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Serialize)]
pub struct Category {
    traits: Vec<Trait>,
    pub dirPath: path::PathBuf,
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct Trait {
    title: String,
    path: path::PathBuf,
    elo: i64,
}
impl Category {
    pub fn new(dp: path::PathBuf) -> Self {
        match get_traits(&dp) {
            Ok(t) => Category {
                traits: t,
                dirPath: dp,
            },
            Err(err) => panic!("{}", err),
        }
    }
    pub fn best_matchup(&self) -> (usize, usize) {
        let mut best: (usize, usize) = (0, 0);
        let mut l: f64 = 1.01f64;
        let length: usize = self.traits.len();
        for n in 0..length {
            for x in (n + 1)..length {
                let s = self.spread(&(n, x));
                /*                 println!(
                    "\n{}\n{}\n{:#?}",
                    self.traits[n].title, self.traits[x].title, s
                ); */
                if s.2 < l {
                    l = s.2;
                    best = (n, x);
                }
            }
        }
        if best.0 == best.1 {
            panic!("Indices match");
        }
        (best.0, best.1)
    }
    pub fn spread(&self, indices: &(usize, usize)) -> (f64, f64, f64) {
        let p: (f64, f64) =
            rating_algo::predict_outcome((self.traits[indices.0].elo, self.traits[indices.1].elo));
        (p.0, p.1, (p.0 - p.1).abs())
    }
    pub fn assign_rarities(&mut self, make_json: bool) -> Result<(), Box<dyn Error>> {
        fn rename_file(path: &path::PathBuf, rank: usize) {
            let path_str: &str = path.to_str().unwrap();
            let mut pieces: Vec<String> = path_str.split(".").map(|s| String::from(s)).collect();
            let sec_2_last = pieces.len() - 2;
            let prev_name = &pieces[sec_2_last];
            if !prev_name.ends_with(format!("_{}", rank).as_str()) {
                pieces[sec_2_last] = format!("{}_{}", prev_name, rank);
                let new = &pieces.join(".");
                println!("prev: {}\nnew: {}", path_str, new);
                fs::rename(path_str, new).expect("Could not rank file");
            }
        }
        self.traits.sort_by(|a, b| b.elo.cmp(&a.elo));
        if make_json {
            let mut cursor: path::PathBuf = (&self.dirPath).to_path_buf();
            cursor.push("rankings.json");
            to_writer_pretty(&fs::File::create(&cursor)?, &self)?;
        } else {
            for (rank, t) in (&self.traits).iter().enumerate() {
                let mut cursor = (&self.dirPath).to_path_buf();
                cursor.push(&t.title);
                if !cursor.is_file() {
                    panic!("File not found: {}", &t.title);
                }
                rename_file(&cursor, rank + 1);
            }
        }
        Ok(())
    }
    pub fn rank(&mut self, config: &mut setup::Config) {
        loop {
            let best: (usize, usize) = self.best_matchup();
            let p: (f64, f64, f64) = self.spread(&best);

            let mut a_wins: bool = false;
            if p.2 > config.settings.precision {
                println!("All Traits Have Been Rated. Rename files or make JSON?");
                match match Select::with_theme(&ColorfulTheme::default())
                    .item("Rename Files")
                    .item("Make JSON")
                    .item("Do Nothing")
                    .default(2)
                    .interact()
                    .expect("Could not read input")
                {
                    0 => self.assign_rarities(false),
                    1 => self.assign_rarities(true),
                    2 => Ok(()),
                    _ => panic!("Invalid input"),
                } {
                    Ok(_) => println!("Successfully ranked traits"),
                    Err(e) => panic!("{}", e),
                }
                break;
            }
            if let Some(map) = config.history.get(&self.traits[best.0].title) {
                if let Some(winner) = map.get(&self.traits[best.1].title) {
                    if winner == &self.traits[best.0].title {
                        a_wins = true;
                    } else {
                        a_wins = false;
                    }
                } else {
                    println!("Pick the more rare one:");
                    let choice = Select::with_theme(&ColorfulTheme::default())
                        .item(&self.traits[best.0].title)
                        .item(&self.traits[best.1].title)
                        .item("Save and exit")
                        .default(2)
                        .interact()
                        .expect("Could not read input");
                    match choice {
                        0 => {
                            a_wins = true;
                        }
                        1 => {
                            a_wins = false;
                        }
                        2 => {
                            break;
                        }
                        _ => {
                            panic!("Invalid input");
                        }
                    }
                }
            } else if let Some(map) = config.history.get(&self.traits[best.1].title) {
                if let Some(winner) = map.get(&self.traits[best.0].title) {
                    if winner == &self.traits[best.0].title {
                        a_wins = true;
                    } else {
                        a_wins = false;
                    }
                } else {
                    println!("Pick the more rare one:");
                    let choice = Select::with_theme(&ColorfulTheme::default())
                        .item(&self.traits[best.0].title)
                        .item(&self.traits[best.1].title)
                        .item("Save and exit")
                        .default(2)
                        .interact()
                        .expect("Could not read input");
                    match choice {
                        0 => {
                            a_wins = true;
                        }
                        1 => {
                            a_wins = false;
                        }
                        2 => {
                            break;
                        }
                        _ => {
                            panic!("Invalid input");
                        }
                    }
                }
            } else {
                println!("Pick the more rare one:");
                let choice = Select::with_theme(&ColorfulTheme::default())
                    .item(&self.traits[best.0].title)
                    .item(&self.traits[best.1].title)
                    .item("Save and exit")
                    .default(2)
                    .interact()
                    .expect("Could not read input");
                match choice {
                    0 => {
                        a_wins = true;
                    }
                    1 => {
                        a_wins = false;
                    }
                    2 => {
                        break;
                    }
                    _ => {
                        panic!("Invalid input");
                    }
                }
            }

            if a_wins {
                self.traits[best.0].elo =
                    rating_algo::update_rating(&self.traits[best.0].elo, &1f64, &p.0);
                self.traits[best.1].elo =
                    rating_algo::update_rating(&self.traits[best.1].elo, &0f64, &p.1);
                config
                    .history
                    .entry((&self.traits[best.0].title).to_string())
                    .or_insert(HashMap::new())
                    .insert(
                        (&self.traits[best.1].title).to_string(),
                        (&self.traits[best.0].title).to_string(),
                    );
            } else {
                self.traits[best.0].elo =
                    rating_algo::update_rating(&self.traits[best.0].elo, &0f64, &p.0);
                self.traits[best.1].elo =
                    rating_algo::update_rating(&self.traits[best.1].elo, &1f64, &p.1);
                config
                    .history
                    .entry((&self.traits[best.0].title).to_string())
                    .or_insert(HashMap::new())
                    .insert(
                        (&self.traits[best.1].title).to_string(),
                        (&self.traits[best.1].title).to_string(),
                    );
            }
            setup::save_progress(&config).expect("Couldnt save progress");
        }
    }
}
fn get_traits(path: &path::PathBuf) -> Result<Vec<Trait>, Box<dyn Error>> {
    if fs::metadata(path)?.is_dir() {
        let mut t: Vec<Trait> = Vec::new();

        for p in fs::read_dir(path)?.map(|elem| elem.unwrap().path()) {
            let file_name: &str = p.file_name().unwrap().to_str().unwrap();
            if file_name
                .split(".")
                .filter(|&elem| elem.len() > 0)
                .collect::<Vec<&str>>()
                .len()
                > 1
                && !&file_name.to_lowercase().ends_with("json")
            {
                t.push(Trait {
                    title: String::from(file_name),
                    path: p,
                    elo: 400,
                })
            }
        }
        Ok(t)
    } else {
        panic!("Path is not a directory: {:#?}", path)
    }
}
pub fn get_cats_deep(path: &path::PathBuf) -> Result<Vec<Category>, Box<dyn Error>> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }
    let mut dirs: Vec<path::PathBuf> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let file: path::PathBuf = entry?.into_path();
        if file.is_file() {
            match file.parent() {
                Some(parent) => {
                    let p: path::PathBuf = parent.to_path_buf();
                    if !dirs.contains(&p) {
                        dirs.push(p)
                    }
                }
                None => {}
            }
        }
    }
    let cats: Vec<Category> = dirs.iter().map(|c| Category::new(c.clone())).collect();
    Ok(cats)
}
