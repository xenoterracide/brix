use crate::args;
use clap::{self, crate_authors, crate_version, App, AppSettings, Arg};

const USAGE: &str = "
    brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
    brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
    brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
";

/// Creates the clap application and sets args
pub fn app() -> App<'static, 'static> {
    let mut app = App::new("brix")
        .author(crate_authors!())
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

    app
}

fn arg_language() -> Arg<'static, 'static> {
    const HELP: &str = "The programming language you're generating code for. Directory under -d";
    Arg::with_name(args::LANGUAGE)
        .value_name("language")
        .help(HELP)
        .required(true)
}

fn arg_config_name() -> Arg<'static, 'static> {
    const HELP: &str = "The type of code you're generating e.g controller, also the name of the config file without the extension";
    Arg::with_name(args::CONFIG_NAME)
        .value_name("config name")
        .help(HELP)
        .required(true)
}

fn arg_project() -> Arg<'static, 'static> {
    const HELP: &str = "The name of the project you're generating code for";
    Arg::with_name(args::PROJECT)
        .value_name("project")
        .help(HELP)
        .required(true)
}

fn arg_module() -> Arg<'static, 'static> {
    const HELP: &str = "The name of the module to be created within the project";
    Arg::with_name(args::MODULE)
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
    Arg::with_name(args::CONFIG_DIR)
        .value_name("config dir")
        .help(HELP)
        .long("config-dir")
        .short("d")
}
