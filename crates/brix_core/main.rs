use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

use clap::ArgMatches;
use colored::*;

use brix_cli;
use brix_config_loader::YamlConfigParser;
use brix_config_loader::{ConfigLoader, ParserList};
use brix_errors::BrixError;

mod app;
mod args;
mod config;
mod util;
use config::Config;

type Result<T> = std::result::Result<T, BrixError>;

fn main() {
    if let Err(err) = args::clap_matches().and_then(next) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn next(matches: ArgMatches<'static>) -> Result<()> {
    let config = Config::new(matches);

    let config_root = Path::new(&config.config_dir);
    let language_dir = Path::new(&config.language);
    let module_dir = config_root.join(language_dir);

    let found_module = module_from_config(&module_dir, &config);
    if found_module.is_err() {
        brix_cli::brix_error(found_module.unwrap_err());
        process::exit(2);
    }

    let declaration = found_module.unwrap();
    let mut file = File::open(declaration.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let processed = brix_processor::process(config.module.clone(), contents)?;
    let parsers: ParserList = vec![Box::new(YamlConfigParser {})];
    let mut loader = ConfigLoader::new(parsers);
    let commands = loader.load(&declaration, &processed).or_else(|err| {
        return Err(BrixError::with(&format!(
            "Error loading config at '{}':\n{}",
            util::display_path(&declaration.to_string_lossy()),
            err
        )));
    })?;
    println!(
        "{} {}",
        "CONFIG".bright_blue(),
        util::display_path(&declaration.to_string_lossy())
    );

    for (command, args) in commands.into_iter() {
        println!("{} {}", "RUNNING".green(), command.name());
        if let Err(err) = command.run(args) {
            eprintln!(
                "Error running {} command in '{}'",
                command.name(),
                declaration.display()
            );
            eprintln!("{}", err);
            process::exit(2);
        }
    }

    println!("----------\n{}", "DONE!".bright_green());
    process::exit(0);
}

fn module_from_config(dir: &PathBuf, config: &Config) -> Result<PathBuf> {
    let declaration = search_for_module_declaration(dir.to_str().unwrap(), &config.config_name)?;

    if declaration.is_none() {
        brix_cli::error_and_quit(&format!(
            "Could not find module declaration for '{}' in {}",
            config.config_name,
            util::display_path(&dir.to_string_lossy())
        ));
    }

    Ok(declaration.unwrap())
}

fn search_for_module_declaration(path: &str, name: &str) -> Result<Option<PathBuf>> {
    let paths = fs::read_dir(path)?;
    let mut results = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let stem = path.file_stem().unwrap();
            if name == stem {
                results.push(path);
            }
        }
    }

    // For now, just select the first match, but in the future be aware
    // that the user might specify an extension they want to use other another
    if results.len() > 0 {
        return Ok(Some(results.get(0).unwrap().to_path_buf()));
    }

    Ok(None)
}
