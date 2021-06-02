use clap::ArgMatches;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::Read;

mod app;
mod args;
mod config;
use config::Config;

use brix_processor;

fn main() {
    if let Err(err) = args::clap_matches().and_then(next) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn next(matches: ArgMatches<'static>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(matches);
    println!("{}", config);

    if let Err(e) = module_from_config(&config) {
        eprintln!("{}", e);
    }
    
    process::exit(0);
}

pub fn module_from_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_root = Path::new(&config.config_dir);
    let language_dir = Path::new(&config.language);
    let path = config_root.join(language_dir).join("module.yaml");

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = brix_processor::process(config.module.clone(), contents)?;
    println!("{}", result);

    Ok(())
}