mod rankable;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config: rankable::setup::Config = rankable::setup::get_config();
    let cats: Vec<rankable::Category> = rankable::get_cats_deep(&config.settings.trait_path)?;
    let mut counter = 0;
    let length = cats.len();
    for mut c in cats {
        counter += 1;
        println!(
            "\n\nCurrently Ranking Category {} / {}\nPath: {:#?}\n\n",
            counter, length, &c.dir_path
        );
        c.rank(&mut config);
    }

    Ok(())
}
