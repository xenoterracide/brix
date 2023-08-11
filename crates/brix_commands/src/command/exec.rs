// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Contains [ExecCommand].

use execute::{shell, Execute};
use std::process::{ExitStatus, Stdio};
use validator::Validate;

use crate::command::{Command, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::{BrixError, BrixErrorKind};

use colored::*;
use log::{debug, error, info, trace};

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

/// The Brix exec command
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
            exec_command.stdout(Stdio::piped()).stderr(Stdio::piped());

            info!("[[ {} ]]", command.bold());
            let output = exec_command.execute_output().unwrap();

            let stdout = String::from_utf8(output.stdout).unwrap();

            if use_stdout {
                println!("{}", stdout);
            } else {
                trace!("{}", stdout);
            }

            if !output.status.success() {
                error!("[[ {} ]]", command.bold());
                return Err(BrixError {
                    kind: None,
                    message: String::from_utf8(output.stderr).unwrap(),
                });
            }
        }

        Ok(())
    }

    fn name(&self) -> String {
        String::from("exec")
    }
}
