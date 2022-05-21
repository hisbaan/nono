mod cli;

use clap::{CommandFactory, Parser};
use dialoguer::Confirm;
use std::fs::metadata;
use std::path::PathBuf;

use cli::Cli;

fn main() {
    let args = Cli::parse();

    if std::env::consts::OS == "windows" {
        let mut cmd = Cli::command();
        let error = cmd.error(
            clap::ErrorKind::DisplayHelp,
            "Windows is not currently supported. Please use a *nix based operating system (linux, macOS, *bsd, etc.)"
        );
        clap::Error::exit(&error);
    }

    let command: Vec<&str> = args.command.iter().map(|s| &**s).collect();

    match command[0] {
        "mv" => { undo_mv(command, args.no_confirm) },
        _ => {},
    }
}

fn undo_mv(command: Vec<&str>, no_confirm: bool) {
    let mut flags: Vec<&str> = vec![];

    for item in &command {
        if item.chars().collect::<Vec<char>>()[0] == '-' {
            flags.push(item);
        }
    }

    if !flags.is_empty() {
        let mut cmd = Cli::command();
        let error = cmd.error(
            clap::ErrorKind::ValueValidation,
            "'mv' flags are not currently supported"
        );
        clap::Error::exit(&error);
    }

    // TODO if the final argument is a directory, move all arguments except the last from the final to the last
    // for example, whoops mv file1 file2 file3 dir1 would result in multiple commands:
    // mv dir1/file1 file1
    // mv dir1/file2 file2
    // mv dir1/file3 file3

    // TODO construct command to be executed then ask the user if they would like to do so


    // if the command is `mv [OPTION]... SOURCE... DIRECTORY`
    if metadata(command[command.len() - 1]).unwrap().is_dir() {
        // Get the last argument.
        let dir = command[command.len() - 1];
        // Loop through all but the first and last arguements.
        for i in 1..command.len() - 1 {
            // Get just the file name (instead of the entire path to the file)
            let file_name = command[i].split("/").last().unwrap();
            let file = PathBuf::from(dir).join(file_name);

            // Check that the files exist
            if !file.is_file() && !file.is_dir() {
                let mut cmd = Cli::command();
                let error = cmd.error(
                    clap::ErrorKind::ValueValidation,
                    format!("File '{}' does not exist", file.into_os_string().into_string().unwrap())
                );
                clap::Error::exit(&error);
            }

            // Prompt the user for each command.
            if no_confirm || Confirm::new()
                .with_prompt(format!(
                    "Run 'mv {} {}'?",
                    file.to_owned().into_os_string().into_string().unwrap(),
                    command[i]
                ))
                .wait_for_newline(true)
                .default(true)
                .interact()
                .unwrap()
            {
                // Move the file from the new location back to the old location.
                std::fs::rename(file, command[i]).expect("Error running move command");
            }
        }
    }
}
