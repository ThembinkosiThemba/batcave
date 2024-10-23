use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use colored::*;
use log::error;

use crate::{
    help::handle_help,
    shell::Shell,
    system::system_info,
    utils::{expand_env_vars, tokenize_command},
};

pub fn execute_command(command: &str, shell: &mut Shell) -> String {
    shell.start_command_timer();
    let result = execute_command_internal(command, shell);
    if let Some(duration) = shell.end_command_timer() {
        if duration > 1.0 {
            println!("Command took {:.2}s", duration);
        }
    }
    result
}

/// This function is used for processing and executing user commands
/// Key steps include
/// 1. Expand environment variables in the command
/// 2. Tokenize the command
/// 3. Check for and expand aliases
/// 4. Match against built-in commands
/// 5. If not a built in command, exectute as an external command
pub fn execute_command_internal(command: &str, shell: &mut Shell) -> String {
    let command = expand_env_vars(command, shell);

    let mut parts: Vec<String> = tokenize_command(&command);

    if parts.is_empty() {
        return String::from("No command entered");
    }

    if let Some(alias_command) = shell.get_alias(&parts[0]) {
        parts = tokenize_command(alias_command);
    }

    match parts[0].as_str() {
        "systeminfo" => toggle_system_info(&parts[1..], shell),
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
        "set-default" => set_as_default_shell(),
        "remove-default" => remove_default_shell(),
        _ => execute_external_command(&parts),
    }
}

fn execute_external_command(parts: &[String]) -> String {
    let mut command = Command::new(&parts[0]);
    command
        .args(&parts[1..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

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

fn toggle_system_info(args: &[String], shell: &mut Shell) -> String {
    match args.get(0).map(|s| s.as_str()) {
        Some("on") => {
            shell.set_show_system_info(true);
            format!("{}System info display enabled{}", "[".green(), "]".green())
        }
        Some("off") => {
            shell.set_show_system_info(false);
            format!("{}System info display disabled{}", "[".green(), "]".green())
        }
        Some("status") => {
            let status = if shell.get_show_system_info() {
                "enabled".green()
            } else {
                "disabled".red()
            };
            format!("System info display is {}", status)
        }
        _ => {
            String::from("Usage: systeminfo [on|off|status] - Configure system information display")
        }
    }
}

fn set_as_default_shell() -> String {
    let shell_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("batcave"));
    let shell_path_str = shell_path.to_string_lossy();

    let shells = fs::read_to_string("/etc/shells").unwrap_or_default();
    if !shells.contains(&*shell_path_str) {
        let status = Command::new("sudo")
            .args(&[
                "sh",
                "-c",
                &format!("echo '{}' >> /etc/shells", shell_path_str),
            ])
            .status();

        if status.is_err() {
            return format!(
                "{}Failed to add Batcave to /etc/shells{}",
                "[".red(),
                "]".red()
            );
        }
    }

    let status = Command::new("chsh").args(&["-s", &shell_path_str]).status();

    match status {
        Ok(_) => format!("{}Batcave set as default shell. Please log out and back in for changes to take effect{}", 
            "[".green(), "]".green()),
        Err(e) => format!("{}Failed to set Batcave as default shell: {}{}", 
            "[".red(), e, "]".red())
    }
}

fn remove_default_shell() -> String {
    // Change shell back to bash
    let status = Command::new("chsh").args(&["-s", "/bin/bash"]).status();

    match status {
        Ok(_) => format!("{}Default shell reset to bash. Please log out and back in for changes to take effect{}", 
            "[".green(), "]".green()),
        Err(e) => format!("{}Failed to reset default shell: {}{}", 
            "[".red(), e, "]".red())
    }
}

fn change_directory(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            if let Err(e) = env::set_current_dir(path) {
                error!("Failed to change directory: {}", e);
                format!(
                    "{}Failed to change directory: {}{}",
                    "[".red(),
                    e,
                    "]".red()
                )
            } else {
                format!(
                    "{}Changed to directory: {}{}",
                    "[".green(),
                    path,
                    "]".green()
                )
            }
        }
        None => {
            error!("cd: missing argument");
            format!("{}cd: missing argument{}", "[".red(), "]".red())
        }
    }
}

fn handle_pushd(args: &[String], shell: &mut Shell) -> String {
    if let Some(dir) = args.get(0) {
        let current = env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        shell.push_dir(current);
        change_directory(Some(&dir.as_str()))
    } else {
        "pushd: missing directory argument".to_string()
    }
}

fn handle_popd(shell: &mut Shell) -> String {
    if let Some(dir) = shell.pop_dir() {
        change_directory(Some(&dir.as_str()))
    } else {
        "popd: directory stack empty".to_string()
    }
}

fn handle_jobs(_shell: &mut Shell) -> String {
    let processes = std::process::Command::new("ps")
        .args(&["aux"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_else(|_| "Failed to get process list".to_string());
    processes
}

fn list_directory(path: Option<&&str>) -> String {
    let path = path.map(Path::new).unwrap_or_else(|| Path::new("."));
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    format!("{}", name.blue())
                } else {
                    name
                }
            })
            .collect::<Vec<_>>()
            .join("  "),
        Err(e) => {
            error!("Failed to list directory: {}", e);
            format!("{}Failed to list directory: {}{}", "[".red(), e, "]".red())
        }
    }
}

fn handle_history(shell: &Shell) -> String {
    shell
        .history
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{:5} {}", i + 1, cmd))
        .collect::<Vec<_>>()
        .join("\n")
}

fn create_directory(path: Option<&&str>) -> String {
    match path {
        Some(path) => {
            if let Err(e) = fs::create_dir(path) {
                error!("Failed to create directory: {}", e);
                format!(
                    "{}Failed to create directory: {}{}",
                    "[".red(),
                    e,
                    "]".red()
                )
            } else {
                format!("{}Directory created: {}{}", "[".green(), path, "]".green())
            }
        }
        None => {
            error!("mkdir: missing argument");
            format!("{}mkdir: missing argument{}", "[".red(), "]".red())
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
                    format!(
                        "{}Failed to remove directory: {}{}",
                        "[".red(),
                        e,
                        "]".red()
                    )
                } else {
                    format!(
                        "{}Directory removed: {}{}",
                        "[".green(),
                        path.display(),
                        "]".green()
                    )
                }
            } else {
                if let Err(e) = fs::remove_file(path) {
                    error!("Failed to remove file: {}", e);
                    format!("{}Failed to remove file: {}{}", "[".red(), e, "]".red())
                } else {
                    format!(
                        "{}File removed: {}{}",
                        "[".green(),
                        path.display(),
                        "]".green()
                    )
                }
            }
        }
        None => {
            error!("rm: missing argument");
            format!("{}rm: missing argument{}", "[".red(), "]".red())
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

pub fn handle_command_not_found(command: &str, shell: &Shell) -> String {
    if let Some(suggestion) = shell.suggest_command(command) {
        format!(
            "Command '{}' not found. Did you mean '{}'?\nYou can run: {} {}",
            command.red(),
            suggestion.green(),
            "help".bright_blue(),
            suggestion.bright_blue()
        )
    } else {
        format!(
            "Command '{}' not found. Try 'help' for a list of commands.",
            command.red()
        )
    }
}
