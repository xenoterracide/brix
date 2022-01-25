// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::Path;

use clap::{self, crate_authors, crate_version, App, AppSettings, Arg};

const USAGE: &str = "
    brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
    brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
    brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
";

// Positional arguments
pub const LANGUAGE: &str = "LANGUAGE";
pub const CONFIG_NAME: &str = "CONFIG_NAME";
pub const PROJECT: &str = "PROJECT";
pub const MODULE: &str = "MODULE";

// Flags
pub const CONFIG_DIR: &str = "CONFIG_DIR";
pub const WORKDIR: &str = "WORKDIR";
pub const LOG_LEVEL: &str = "LOG_LEVEL";

/// Creates the clap application and sets args
pub fn app() -> App<'static, 'static> {
    let mut app = App::new("brix")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .max_term_width(100)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::ArgRequiredElseHelp)
        .usage(USAGE)
        .help_message("Prints this message");

    app = app.arg(arg_language());
    app = app.arg(arg_config_name());
    app = app.arg(arg_project());
    app = app.arg(arg_module());
    app = app.arg(flag_config_dir());
    app = app.arg(flag_log_level());
    app = app.arg(flag_workdir());

    app
}

fn arg_language() -> Arg<'static, 'static> {
    const HELP: &str = "The programming language you're generating code for. Directory under -d";
    Arg::with_name(LANGUAGE)
        .value_name("language")
        .help(HELP)
        .required(true)
}

fn arg_config_name() -> Arg<'static, 'static> {
    const HELP: &str = "The type of code you're generating e.g controller, also the name of the config file without the extension";
    Arg::with_name(CONFIG_NAME)
        .value_name("config name")
        .help(HELP)
        .required(true)
}

fn arg_project() -> Arg<'static, 'static> {
    const HELP: &str = "The name of the project you're generating code for";
    Arg::with_name(PROJECT)
        .value_name("project")
        .help(HELP)
        .required(true)
}

fn arg_module() -> Arg<'static, 'static> {
    const HELP: &str = "The name of the module to be created within the project";
    Arg::with_name(MODULE)
        .value_name("module")
        .help(HELP)
        .required(true)
}

fn flag_config_dir() -> Arg<'static, 'static> {
    const HELP: &str = "
Directory path from the current working directory.
Templates and configs are looked up relative to here.
If the config isn't found here, then ~/.config/brix will be searched
    ";
    Arg::with_name(CONFIG_DIR)
        .value_name("config dir")
        .help(HELP)
        .long("config-dir")
        .short("d")
        .takes_value(true)
        .validator(is_valid_path)
}

fn flag_workdir() -> Arg<'static, 'static> {
    const HELP: &str =
        "The current working directory to use. Defaults to the directory where brix is run from";
    Arg::with_name(WORKDIR)
        .value_name("workdir")
        .help(HELP)
        .long("workdir")
        .short("w")
        .takes_value(true)
        .validator(is_valid_path)
}

fn flag_log_level() -> Arg<'static, 'static> {
    const HELP: &str = "The log level to use while running a command";
    Arg::with_name(LOG_LEVEL)
        .value_name("log level")
        .help(HELP)
        .long("log-level")
        .takes_value(true)
        .default_value("off")
        .possible_values(&["off", "error", "warn", "info", "debug", "trace"])
}

fn is_valid_path(v: String) -> Result<(), String> {
    let path = Path::new(&v);
    if path.exists() {
        Ok(())
    } else {
        Err(format!("The directory {} does not exist", v))
    }
}
