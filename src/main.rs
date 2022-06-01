mod rankable;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde_json::to_string_pretty;
use std::io::Write;
use std::{collections::HashMap, env, error::Error, fs, io, path};

fn main() -> std::io::Result<()> {
    let mut config: rankable::setup::Config = rankable::setup::initialize();
    let mut cats = rankable::Category::new(config.settings.trait_path.clone());
    cats.rank(&mut config);
    Ok(())
}
