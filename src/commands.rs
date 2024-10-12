use std::{env, fs, path::Path, process::{Command, Stdio}};

use colored::*;
use log::error;

use crate::{
    help::handle_help, shell::Shell, system::system_info, utils::{expand_env_vars, tokenize_command}
};

/// This function is used for processing and executing user commands
/// Key steps include
/// 1. Expand environment variables in the command
/// 2. Tokeenize the command
/// 3. Check for and expand aliases
/// 4. Match against built-in commands
/// 5. If not a built in command, exectute as an external command

pub fn execute_command(command: &str, shell: &mut Shell) -> String {
    let command = expand_env_vars(command, shell);

    let mut parts: Vec<String> = tokenize_command(&command);

    if parts.is_empty() {
        return String::from("No command entered");
    }

    if let Some(alias_command) = shell.get_alias(&parts[0]) {
        parts = tokenize_command(alias_command);
    }

    match parts[0].as_str() {
        "echo" => handle_echo(&parts[1..], shell),
        "pwd" => env::current_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| String::from("Failed to get current directory")),
        "cd" => change_directory(parts.get(1).map(|s| s.as_str()).as_ref()),
        "ls" => list_directory(parts.get(1).map(|s| s.as_str()).as_ref()),
        "mkdir" => create_directory(parts.get(1).map(|s| s.as_str()).as_ref()),
        "rm" => remove_file_or_directory(parts.get(1).map(|s| s.as_str()).as_ref()),
        "touch" => create_file(parts.get(1).map(|s| s.as_str()).as_ref()),
        "alias" => handle_alias(&parts[1..], shell),
        "export" => handle_export(&parts[1..], shell),
        "env" => shell
            .env_vars
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("\n"),
        "info" => system_info(),
        "help" => handle_help(&parts[1..]),
        _ => execute_external_command(&parts),
    }
}

fn execute_external_command(parts: &[String]) -> String {
    let mut command = Command::new(&parts[0]);
    command
        .args(&parts[1..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).into_owned()
            } else {
                String::from_utf8_lossy(&output.stderr).into_owned()
            }
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}

fn change_directory(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            if let Err(e) = env::set_current_dir(path) {
                error!("Failed to change directory: {}", e);
                format!("Failed to change directory: {}", e)
            } else {
                String::new()
            }
        }
        None => {
            error!("cd: missing argument");
            String::from("cd: missing argument")
        }
    }
}

fn list_directory(path: Option<&&str>) -> String {
    let path = path.map(Path::new).unwrap_or_else(|| Path::new("."));
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    name.blue().to_string()
                } else {
                    name
                }
            })
            .collect::<Vec<_>>()
            .join("  "),
        Err(e) => {
            error!("Failed to list directory: {}", e);
            format!("Failed to list directory: {}", e)
        }
    }
}

fn create_directory(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            if let Err(e) = fs::create_dir(path) {
                error!("Failed to create directory: {}", e);
                format!("Failed to create directory: {}", e)
            } else {
                format!("Directory created: {}", path)
            }
        }
        None => {
            error!("mkdir: missing argument");
            String::from("mkdir: missing argument")
        }
    }
}

fn remove_file_or_directory(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            let path = Path::new(path);
            if path.is_dir() {
                if let Err(e) = fs::remove_dir_all(path) {
                    error!("Failed to remove directory: {}", e);
                    format!("Failed to remove directory: {}", e)
                } else {
                    format!("Directory removed: {}", path.display())
                }
            } else {
                if let Err(e) = fs::remove_file(path) {
                    error!("Failed to remove file: {}", e);
                    format!("Failed to remove file: {}", e)
                } else {
                    format!("File removed: {}", path.display())
                }
            }
        }
        None => {
            error!("rm: missing argument");
            String::from("rm: missing argument")
        }
    }
}

fn create_file(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            if let Err(e) = fs::File::create(path) {
                error!("Failed to create file: {}", e);
                format!("Failed to create file: {}", e)
            } else {
                format!("File created: {}", path)
            }
        }
        None => {
            error!("touch: missing argument");
            String::from("touch: missing argument")
        }
    }
}

fn handle_echo(args: &[String], shell: &Shell) -> String {
    args.iter()
        .map(|arg| expand_env_vars(arg, shell))
        .collect::<Vec<_>>()
        .join(" ")
}

fn handle_alias(args: &[String], shell: &mut Shell) -> String {
    if args.is_empty() {
        return shell
            .aliases
            .iter()
            .map(|(name, command)| format!("{}='{}'", name, command))
            .collect::<Vec<_>>()
            .join("\n");
    }

    let alias_str = args.join(" ");
    if let Some(equals_pos) = alias_str.find('=') {
        let name = alias_str[..equals_pos].trim().to_string();
        let command = alias_str[equals_pos + 1..]
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        shell.add_alias(name.clone(), command.clone());
        format!("Added alias: {}='{}'", name, command)
    } else {
        "Usage: alias name=command".to_string()
    }
}

fn handle_export(args: &[String], shell: &mut Shell) -> String {
    if args.is_empty() {
        return "Usage: export NAME=value".to_string();
    }

    for arg in args {
        if let Some(equals_pos) = arg.find('=') {
            let name = arg[..equals_pos].to_string();
            let value = arg[equals_pos + 1..].to_string();
            shell.set_env(name, value);
        }
    }
    String::new()
}
