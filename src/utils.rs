use std::{
    env, fs,
    io::{self, Error as IoError},
    path::Path,
};

use colored::*;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};

use crate::shell::Shell;

pub fn print_banner() {
    println!(
        "{}",
        r#"
        ▄▄▄█████▓ ██░ ██ ▓█████     ▄▄▄▄    ▄▄▄     ▄▄▄█████▓ ▄████▄   ▄▄▄    ██▒   █▓▓█████ 
        ▓  ██▒ ▓▒▓██░ ██▒▓█   ▀    ▓█████▄ ▒████▄   ▓  ██▒ ▓▒▒██▀ ▀█  ▒████▄ ▓██░   █▒▓█   ▀ 
        ▒ ▓██░ ▒░▒██▀▀██░▒███      ▒██▒ ▄██▒██  ▀█▄ ▒ ▓██░ ▒░▒▓█    ▄ ▒██  ▀█▄▓██  █▒░▒███   
        ░ ▓██▓ ░ ░▓█ ░██ ▒▓█  ▄    ▒██░█▀  ░██▄▄▄▄██░ ▓██▓ ░ ▒▓▓▄ ▄██▒░██▄▄▄▄██▒██ █░░▒▓█  ▄ 
          ▒██▒ ░ ░▓█▒░██▓░▒████▒   ░▓█  ▀█▓ ▓█   ▓██▒ ▒██▒ ░ ▒ ▓███▀ ░ ▓█   ▓██▒▒▀█░  ░▒████▒
          ▒ ░░    ▒ ░░▒░▒░░ ▒░ ░   ░▒▓███▀▒ ▒▒   ▓▒█░ ▒ ░░   ░ ░▒ ▒  ░ ▒▒   ▓▒█░░ ▐░  ░░ ▒░ ░
            ░     ▒ ░▒░ ░ ░ ░  ░   ▒░▒   ░   ▒   ▒▒ ░   ░      ░  ▒     ▒   ▒▒ ░░ ░░   ░ ░  ░
          ░       ░  ░░ ░   ░       ░    ░   ░   ▒    ░      ░          ░   ▒     ░░     ░   
                  ░  ░  ░   ░  ░    ░            ░  ░        ░ ░            ░  ░   ░     ░  ░
                                      ░                      ░                     ░         
        "#
        .bright_blue()
    );

    println!(
        "{}",
        "Welcome to the Batcave Terminal. Proceed with caution.".bright_yellow()
    );

    println!();
    println!();
}

pub fn setup_logging() -> io::Result<()> {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|e| IoError::new(io::ErrorKind::NotFound, e))?;
    let log_path = Path::new(&home_dir).join(".batcave.log");

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        file,
    )])
    .map_err(|e| IoError::new(io::ErrorKind::Other, e))?;

    Ok(())
}

// splits command string into tokens
pub fn tokenize_command(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for c in command.chars() {
        match (c, in_quotes, escape_next) {
            ('\\', _, false) => escape_next = true,
            ('"', _, true) => {
                current_token.push('"');
                escape_next = false;
            }
            (c, _, true) => {
                current_token.push('\\');
                current_token.push(c);
                escape_next = false;
            }
            ('"', _, false) => in_quotes = !in_quotes,
            (' ', false, false) => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            (c, _, false) => current_token.push(c),
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

// expand env variables in a string
pub fn expand_env_vars(command: &str, shell: &Shell) -> String {
    let mut result = command.to_string();
    while let Some(start) = result.find('$') {
        if let Some(end) = result[start + 1..].find(|c: char| !c.is_alphanumeric() && c != '_') {
            let var_name = &result[start + 1..start + 1 + end];
            if let Some(value) = shell.get_env(var_name) {
                result.replace_range(start..start + 1 + end, value);
            }
        } else {
            let var_name = &result[start + 1..];
            if let Some(value) = shell.get_env(var_name) {
                result.replace_range(start.., value);
            }
            break;
        }
    }
    result
}

// provides autocomplete candidates for auto-completion
pub fn autocomplete(input: &str, shell: &Shell) -> Vec<String> {
    let mut completions = Vec::new();

    // Complete commands
    let commands = vec![
        "echo", "pwd", "cd", "ls", "mkdir", "rm", "touch", "alias", "export", "env",
    ];
    for cmd in commands {
        if cmd.starts_with(input) {
            completions.push(cmd.to_string());
        }
    }

    // Complete aliases
    for alias in shell.aliases.keys() {
        if alias.starts_with(input) {
            completions.push(alias.to_string());
        }
    }

    // Complete environment variables
    for var in shell.env_vars.keys() {
        if var.starts_with(input) {
            completions.push(format!("${}", var));
        }
    }

    // Complete file system entries
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(input) {
                completions.push(name);
            }
        }
    }

    completions
}
