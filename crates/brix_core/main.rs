use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

use clap::ArgMatches;

use brix_cli;
#[allow(unused_imports)]
use brix_commands;
use brix_config_loader::{self, Command};
use brix_errors::BrixError;
use brix_processor;
use config::Config;

mod app;
mod args;
mod config;
mod util;

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
    let mut file = File::open(declaration)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let processed = brix_processor::process(config.module.clone(), contents)?;
    let command = brix_config_loader::load(&processed);

    // TODO: handle invalid config loading in loader instead of this
    if let Command::Empty = command {
        eprintln!("No command found for config file!");
    }

    let result = match command {
        Command::TemplateAndCopy(source, destination) => {
            template_and_copy(&module_dir, &source, &destination)
        }
        _ => Ok(()),
    };

    if result.is_err() {
        brix_cli::brix_error(result.unwrap_err());
    }

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
        return Ok(Some(results.get(0).unwrap().to_path_buf()));
    }

    Ok(None)
}

// TODO: perhaps put this in a separate module
fn template_and_copy(module_dir: &Path, source: &str, dest: &str) -> Result<()> {
    let source_path = module_dir.join(source);
    let contents = fs::read_to_string(source_path)?;

    let path = Path::new(dest);
    let parent = path.parent().unwrap(); // Fix

    fs::create_dir_all(parent).unwrap();
    let mut file = File::create(dest).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("Done!");
    Ok(())
}
