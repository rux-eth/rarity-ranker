mod rating_algo;
pub mod setup;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;
use std::{collections::HashMap, fs, path};
#[derive(Debug)]
pub struct Category {
    traits: Vec<Trait>,
    dirPath: path::PathBuf,
}
#[derive(Debug)]
struct Trait {
    title: String,
    path: path::PathBuf,
    elo: f64,
}
impl Category {
    pub fn new(dp: path::PathBuf) -> Self {
        println!("{:#?}", dp);

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
        let mut l: f64 = 1f64;
        let length: usize = self.traits.len();
        for n in 0..length {
            for x in (n + 1)..length {
                let s = self.spread(&(n, x));
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
    pub fn rank(&mut self, config: &mut setup::Config) {
        loop {
            let best: (usize, usize) = self.best_matchup();
            let p: (f64, f64, f64) = self.spread(&best);
            println!("{:#?}", best);
            println!("{:#?}", p);

            let mut a_wins: bool = false;
            if p.2 > config.settings.precision {
                break;
            }
            if let Some(map) = config.history.get(&self.traits[best.0].title) {
                if let Some(winner) = map.get(&self.traits[best.1].title) {
                    if winner == &self.traits[best.0].title {
                        a_wins = true;
                    } else {
                        a_wins = false;
                    }
                }
            } else if let Some(map) = config.history.get(&self.traits[best.1].title) {
                if let Some(winner) = map.get(&self.traits[best.0].title) {
                    if winner == &self.traits[best.0].title {
                        a_wins = true;
                    } else {
                        a_wins = false;
                    }
                }
            } else {
                println!("Pick the more rare one:");
                let choice = Select::with_theme(&ColorfulTheme::default())
                    .item(&self.traits[best.0].title)
                    .item(&self.traits[best.1].title)
                    .item("Save and exit")
                    .default(0)
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
            println!("{:#?}", self.traits);
        }
    }
}
fn get_traits(path: &path::PathBuf) -> Result<Vec<Trait>, Box<dyn Error>> {
    println!("{:#?}", path);
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
            {
                t.push(Trait {
                    title: String::from(file_name),
                    path: p,
                    elo: 400.0,
                })
            }
        }
        Ok(t)
    } else {
        panic!("Path is not a directory: {:#?}", path)
    }
}
/* pub fn get_categories(base_path: &path::PathBuf) -> Result<Vec<Category>, Box<dyn Error>> {
    let mut cats: Vec<Category> = Vec::new();
    fn parse_dir_deep(path: &path::PathBuf) -> Result<(), Box<dyn Error>> {
        let all_dirs: bool;
        let filtered: Vec<path::PathBuf> = fs::read_dir(path)
            .expect(format!("Invalid path: {:#?}", path).as_str())
            .map(|p| p.unwrap().path())
            .filter(|p| !p.file_name().unwrap().to_str().unwrap().starts_with("."))
            .collect();
        if filtered.len() < 1 {
            panic!("Directory has no contents: {:#?}", path);
        }
        Ok(())
    }
}
 */
