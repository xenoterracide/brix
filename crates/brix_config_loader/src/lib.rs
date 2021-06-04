mod command;
mod config;
pub use command::Command;

pub fn load(contents: &str) -> Command {
    let config: config::Config = serde_yaml::from_str(contents).unwrap();
    let command = config.get_command();
    command
}
