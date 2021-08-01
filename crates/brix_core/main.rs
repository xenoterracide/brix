use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

use colored::*;

use brix_common::AppContext;
use brix_config_loader::YamlConfigParser;
use brix_config_loader::{ConfigLoader, ParserList};
use brix_errors::BrixError;
use brix_processor::ProcessorCore;

mod util;

type Result<T> = std::result::Result<T, BrixError>;

fn main() {
    if let Err(err) = brix_cli::clap_matches().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(matches: brix_cli::ArgMatches<'static>) -> Result<()> {
    let config = brix_cli::Config::new(matches);

    let config_root = Path::new(&config.config_dir);
    let language_dir = Path::new(&config.language);
    let module_dir = config_root.join(language_dir);

    let found_modules = modules_from_config(&module_dir, &config);
    if found_modules.is_err() {
        eprintln!("{}", found_modules.unwrap_err());
        process::exit(2);
    }

    let declarations = found_modules.unwrap();
    let parsers: ParserList = vec![Box::new(YamlConfigParser {})];
    let mut loader = ConfigLoader::new(parsers, &config);
    let config_file = loader.load(declarations)?;

    // Create the app context
    let processor = ProcessorCore::new();
    let app_context = AppContext { processor };

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

    for (command, args) in commands.into_iter() {
        println!("{} {}", "RUNNING".green(), command.name());
        if let Err(err) = command.run(args, &app_context) {
            eprintln!(
                "Error running {} command in '{}'",
                command.name(),
                config_file.display()
            );
            eprintln!("{}", err);
            process::exit(2);
        }
    }
    let elapsed = start.elapsed();

    println!(
        "----------\n{} in {}ms",
        "DONE!".bright_green(),
        elapsed.as_millis()
    );
    process::exit(0);
}

fn modules_from_config(dir: &PathBuf, config: &brix_cli::Config) -> Result<Vec<PathBuf>> {
    let declarations = search_for_module_declarations(dir.to_str().unwrap(), &config.config_name)?;

    if declarations.len() == 0 {
        return Err(BrixError::with(&format!(
            "Could not find module declaration for '{}' in {}",
            config.config_name,
            util::display_path(&dir.to_string_lossy())
        )));
    }

    Ok(declarations)
}

fn search_for_module_declarations(path: &str, name: &str) -> Result<Vec<PathBuf>> {
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

    Ok(results)
}
