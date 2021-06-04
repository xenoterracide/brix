use clap::ArgMatches;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

mod app;
mod args;
mod config;
use config::Config;

use brix_config_loader::{self, Command};
use brix_processor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = args::clap_matches().and_then(next) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn next(matches: ArgMatches<'static>) -> Result<()> {
    let config = Config::new(matches);

    if let Err(e) = module_from_config(&config) {
        eprintln!("{}", e);
    }

    process::exit(0);
}

fn module_from_config(config: &Config) -> Result<()> {
    let config_root = Path::new(&config.config_dir);
    let language_dir = Path::new(&config.language);
    let module_dir = config_root.join(language_dir);
    let declaration = search_for_module_declaration(module_dir.to_str().unwrap(), &config.config_name)?;

    if declaration.is_none() {
        eprintln!(
            "Could not find module declaration for '{}' in {}",
            config.config_name,
            module_dir.to_string_lossy()
        );
        process::exit(2);
    }

    let mut file = File::open(declaration.unwrap().as_ref())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let processed = brix_processor::process(config.module.clone(), contents)?;
    let command = brix_config_loader::load(&processed);

    // TODO: handle invalid config loading in loader instead of this
    if let Command::Empty = command {
        eprintln!("No command found for config file!");
    }

    match command {
        Command::TemplateAndCopy(source, destination) => {
            template_and_copy(&module_dir, &source, &destination);
        }
        _ => {}
    };

    Ok(())
}

fn search_for_module_declaration(path: &str, name: &str) -> Result<Option<Box<PathBuf>>> {
    let paths = fs::read_dir(path)?;
    let mut results = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let stem = path.file_stem().unwrap();
            let ext = path.extension().unwrap();

            // TODO: replace with supported config files
            if name == stem && (ext == "yml" || ext == "yaml") {
                results.push(path);
            }
        }
    }

    // For now, just select the first match, but in the future be aware
    // that the user might specify an extension they want to use other another
    if results.len() > 0 {
        return Ok(Some(Box::new(results.get(0).unwrap().to_path_buf())));
    }

    Ok(None)
}

// TODO: perhaps put this in a separate module
fn template_and_copy(module_dir: &Path, source: &str, dest: &str) {
    // TODO: also actually do templating and stop unwrapping everywhere
    let source_path = module_dir.join(source);
    let contents = fs::read_to_string(source_path).unwrap();
    
    let path = Path::new(dest);
    let parent = path.parent().unwrap(); // Fix

    fs::create_dir_all(parent).unwrap();
    let mut file = File::create(dest).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("Done!");
}
