// The Batcave Shell is a custom command-line interface implemented in Rust.
// It provides a set of built-in commands, supports external command execution,
// and includes features like environment variable management, command aliases,
// and auto-completion.
mod commands;
mod help;
mod shell;
mod system;
mod utils;

use crate::commands::execute_command;
use crate::shell::{Shell, ShellHelper};
use system::system_info;
use utils::{print_banner, setup_logging};

use std::{env, io};

use colored::*;
use log::info;
use rustyline::{error::ReadlineError, Editor};

fn main() -> io::Result<()> {
    setup_logging()?;
    print_banner();

    let mut shell = Shell::new();
    let helper = ShellHelper::new(&shell);
    let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    rl.set_helper(Some(helper));

    shell.add_alias("ll".to_string(), "ls -la".to_string());
    shell.add_alias("cls".to_string(), "clear".to_string());

    if shell.get_env("SHOW_SYSTEM_INFO").is_none() {
        println!("\n{}", system_info());
        println!(
            "\nWould you like to see this system information every time you start the shell? [Y/n]"
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let show_info = !input.trim().eq_ignore_ascii_case("n");
        shell.set_show_system_info(show_info);
        println!("\nPreference saved. You can change this later using the 'systeminfo' command.");
    } else if shell.get_show_system_info() {
        println!("\n{}", system_info());
    }

    println!();

    loop {
        let current_dir = env::current_dir()?;
        let prompt = format!("🦇 {}> ", current_dir.display())
            .bright_purple()
            .to_string();

        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" {
                    println!("{}", "Exiting the Batcave...".bright_blue());
                    break;
                }

                rl.add_history_entry(input.as_str());
                shell.add_to_history(input.clone());

                let output = execute_command(&input, &mut shell);
                println!("{}", output.bright_white());
                info!("Executed command: {}", input);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
