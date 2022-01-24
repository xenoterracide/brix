// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

use colored::*;

use brix_cli::error as cli_error;
use brix_common::AppContext;
use brix_config_loader::YamlConfigParser;
use brix_config_loader::{ConfigLoader, ParserList};
use brix_errors::BrixError;
use brix_processor::ProcessorCore;
use log::{debug, error, info};
use simple_logger::SimpleLogger;

mod util;

type Result<T> = std::result::Result<T, BrixError>;

fn main() {
    if let Err(err) = brix_cli::clap_matches().and_then(try_main) {
        cli_error!("{}", err);
        process::exit(2);
    }
}

fn try_main(matches: brix_cli::ArgMatches<'static>) -> Result<()> {
    let home_dir = home::home_dir();
    let config = brix_cli::Config::new(home_dir.clone(), matches);

    SimpleLogger::new()
        .with_level(config.log_level)
        .init()
        .unwrap();

    debug!("HOME DIR: {:?}", home_dir);

    let config_root = Path::new(&config.config_dir);
    let language_dir = Path::new(&config.language);
    let module_dir = config_root.join(language_dir);

    let found_modules = modules_from_config(&module_dir, &config);
    if found_modules.is_err() {
        brix_cli::error!("{}", found_modules.unwrap_err());
        process::exit(2);
    }

    let declarations = found_modules.unwrap();
    let parsers: ParserList = vec![Box::new(YamlConfigParser {})];
    let mut loader = ConfigLoader::new(parsers, &config);
    let config_file = loader.load(declarations)?;

    // Create the app context
    let processor = ProcessorCore::new();
    let app_context = AppContext {
        processor,
        config: &config,
    };

    let start = Instant::now();
    let commands = loader.run(&app_context).or_else(|err| {
        return Err(BrixError::with(&format!(
            "Error loading config at '{}':\n{}",
            util::display_path(&config_file.to_string_lossy()),
            err
        )));
    })?;

    println!(
        "{} {}",
        "CONFIG".bright_blue(),
        util::display_path(&config_file.to_string_lossy())
    );

    // Count the number of each type of command and how many times it was run
    let mut map: HashMap<String, (i32, i32)> = HashMap::new();
    for (command, _) in commands.iter() {
        let name = command.name();
        map.insert(name.clone(), (map.get(&name).unwrap_or(&(0, 0)).0 + 1, 0));
    }

    for (command, args) in commands.into_iter() {
        let name = command.name();
        let (total, ran) = *map.get(&name).unwrap();

        println!(
            "{} {} ({}/{})",
            "RUNNING".green(),
            name.bold(),
            ran + 1,
            total,
        );
        if let Err(err) = command.run(args, &app_context) {
            cli_error!(
                "Error running {} command in '{}'",
                command.name(),
                util::display_path(&format!("{}", config_file.display()))
            );
            error!("{}", err);

            process::exit(2);
        }

        map.insert(name, (total, ran + 1));
    }
    let elapsed = start.elapsed();

    println!("----------\n{} in {:#?}", "DONE!".bright_green(), elapsed);
    process::exit(0);
}

fn modules_from_config(dir: &PathBuf, config: &brix_cli::Config) -> Result<Vec<PathBuf>> {
    let declarations = search_for_module_declarations_all(dir.to_str().unwrap(), &config)?;

    if declarations.len() == 0 {
        return Err(BrixError::with(&format!(
            "Could not find module declaration for '{}' in {}",
            config.config_name,
            util::display_path(&dir.to_string_lossy())
        )));
    }

    Ok(declarations)
}

fn search_for_module_declarations_all(
    path: &str,
    config: &brix_cli::Config,
) -> Result<Vec<PathBuf>> {
    let mut current_path = config.workdir.clone();

    loop {
        debug!("Looking for config directory in {:?}", current_path);
        let declarations =
            search_for_module_declarations(&current_path, &path, &config.config_name)?;
        if declarations.len() > 0 {
            return Ok(declarations);
        }

        if &current_path == config.home_dir.as_ref().unwrap() {
            return Err(BrixError::with(&format!(
                "Could not find module declaration for '{}' in {}",
                config.config_name,
                util::display_path(&path)
            )));
        }

        current_path = current_path.parent().unwrap().to_path_buf();
    }
}

fn search_for_module_declarations(
    current_path: &PathBuf,
    path: &str,
    name: &str,
) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    let search_path = current_path.join(path);
    if !search_path.exists() {
        // Should not error if the directory doesn't exist,
        // just return an empty vec of results
        return Ok(vec![]);
    }

    let paths = fs::read_dir(search_path)?;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let stem = path.file_stem().unwrap();
            if name == stem || format!("{}.brix", name) == stem.to_str().unwrap() {
                results.push(path);
            }
        }
    }
    info!("RESULTS: {:?}", results);

    Ok(results)
}
