use clap::Parser;

#[derive(Parser)]
#[clap(author = "Hisbaan Noorani", version = "0.0.1", about = "A CLI utility to undo common commands", long_about = "Whoops is a CLI utility made to undo common commands. Specifically, it is designed to work on most GNU core utilities with the caveat that the original command must not be a distructive one. This means that it will not work on commands like `rm` and may not work properly on any command that overwrites a file. In cases such as these, there is no warranty provided, and no contributor to whoops is responsible for loss of data or other information.")]
pub struct Cli {
    #[clap(help = "The command to undo")]
    pub command: Vec<String>,
    #[clap(
        short = 'n',
        long = "no-confirm",
        help = "Don't use confirmation prompts",
        long_help = "Do not prompt the user for confirmation about each command the program wishes to use."
    )]
    pub no_confirm: bool,
}
