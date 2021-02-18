use std::{
    env,
    error::Error,
    io::stdout,
};

mod app;
mod model;
mod view;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: for "cargo run" to work without additional arguments
    let default_map = String::from("src/resources/map.txt");

    let args: Vec<String> = env::args().collect();
    let map_path = &args.get(1).unwrap_or(&default_map);

    App::new(stdout(), map_path)?.run()?;

    Ok(())
}
