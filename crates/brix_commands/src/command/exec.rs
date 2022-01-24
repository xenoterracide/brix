// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use execute::{shell, Execute};
use std::process::Stdio;
use validator::Validate;

use crate::command::{Command, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::BrixError;

use colored::*;

#[derive(Debug)]
pub struct ExecParams {
    commands: Vec<String>,
    stdout: bool,
}

impl PartialEq for ExecParams {
    fn eq(&self, other: &Self) -> bool {
        return self.commands == other.commands && self.stdout == other.stdout;
    }
}

#[derive(Debug, Validate)]
struct Params {
    #[validate(required, length(min = 1))]
    commands: Option<Vec<String>>,
    stdout: Option<bool>,
}

pub struct ExecCommand {}

impl ExecCommand {
    pub fn new() -> Self {
        ExecCommand {}
    }
}

impl Command for ExecCommand {
    fn run(&self, pcp: ProcessedCommandParams, _app_context: &AppContext) -> Result<(), BrixError> {
        let cp = Params {
            commands: pcp.commands,
            stdout: pcp.stdout,
        };
        cp.validate()?;

        let commands = cp.commands.unwrap();
        let use_stdout = cp.stdout.unwrap_or(true);

        for command in commands.iter() {
            let mut exec_command = shell(command);
            exec_command.stdout(Stdio::piped());

            println!("[[ {} ]]", command.bold());

            let mut output: Option<std::process::Output> = None;
            if use_stdout {
                output = Some(exec_command.execute_output().unwrap());
            } else {
                exec_command.execute().unwrap();
            }

            if use_stdout {
                let stdout = String::from_utf8(output.unwrap().stdout).unwrap();
                // let lines: Vec<&str> = stdout.split("\n").collect();

                // let mut fin = String::new();
                // for line in lines {
                //     if !line.is_empty() {
                //         fin.push_str(&format!("➞ ➞ {}", line))
                //     } else {
                //         fin.push_str("\n");
                //     }
                // }

                println!("{}", stdout);
            }
        }

        Ok(())
    }

    fn name(&self) -> String {
        String::from("exec")
    }
}
