use clap::{arg, Command as ClapCommand};
use colored::*;
use xterm_js_rs::Terminal;

use super::Command;

const TEY_SH_ASCII_LOGO: &'static str = " _                   _     
| |_ ___ _   _   ___| |__  
| __/ _ \\ | | | / __| '_ \\ 
| ||  __/ |_| |_\\__ \\ | | |
 \\__\\___|\\__, (_)___/_| |_|
          |___/             
";

pub struct TeySh {
    command: ClapCommand,
    long_help: String,
    short_help: String,
    long_version: String,
}

impl TeySh {
    pub fn new() -> Self {
        let mut command = ClapCommand::new("tey.sh")
            // General settings
            .color(clap::ColorChoice::Always)
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about(clap::crate_description!())
            //.ignore_errors(true) // Ignore parsing errors as clap would call [std::process::exit] in wasm context which leads to panic
            .disable_version_flag(true) // Ignore due to wasm (custom impl)
            .disable_help_subcommand(true) // Ignore due to wasm (custom impl)
            .disable_help_flag(true) // Ignore due to wasm (custom impl)
            .before_long_help(format!(
                "{}\n{}",
                TEY_SH_ASCII_LOGO.bold().green(),
                format!("v{} by {}", clap::crate_version!(), clap::crate_authors!()).dimmed()
            ))
            .subcommand_required(false)
            // Flags
            .arg(arg!(
                -h --help "Print help"
            ))
            .arg(arg!(
                -V --version "Print version"
            ))
            // Subcommands
            .subcommand(
                ClapCommand::new("tui").about("View my website in a terminal user interface"),
            )
            .subcommand(ClapCommand::new("cli").about("View my website directly in the cli"));

        let long_help = command.render_long_help().ansi().to_string();
        let short_help = command.render_help().ansi().to_string();
        let long_version = command.render_long_version();

        Self {
            command,
            long_help,
            short_help,
            long_version,
        }
    }
}

impl Command for TeySh {
    fn execute(&self, tokens: &Vec<&str>, terminal: &Terminal) -> bool {
        let matches = self.command.clone().try_get_matches_from(tokens);

        if let Ok(matches) = matches {
            if let Some((subcommand, _args)) = matches.subcommand() {
                match subcommand {
                    "tui" => return true, // Switch to tui mode
                    "cli" => todo!("Print cli website content"),
                    _ => panic!("Failed to find a matching subcommand. This is a bug, please verify if all registered subcommands are properly handled in the execute function.")
                }
            } else {
                if matches.get_flag("help") {
                    terminal.write(&self.long_help);
                } else if matches.get_flag("version") {
                    terminal.write(&self.long_version);
                } else {
                    terminal.write(&self.short_help);
                }
            }
        } else {
            terminal.write(&matches.unwrap_err().render().ansi().to_string());
        }

        false
    }
}
