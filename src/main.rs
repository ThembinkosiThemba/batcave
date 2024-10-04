use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use colored::*;
use log::{error, info};
use simple_logger::SimpleLogger;

fn main() -> io::Result<()> {
    SimpleLogger::new().init().unwrap();
    print_banner();

    loop {
        let current_dir = env::current_dir()?;
        print!(
            "{} ",
            format!("🦇 {}> ", current_dir.display())
                .bright_purple()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input == "exit" {
            println!("{}", "Exiting the Batcave...".bright_blue());
            break;
        }

        let output = execute_command(input);
        println!("{}", output.bright_white());
        info!("Executed command: {}", input);
    }
    Ok(())
}

fn execute_command(command: &str) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return String::from("No command entered");
    }

    match parts[0] {
        "echo" => parts[1..].join(" "),
        "pwd" => env::current_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| String::from("Failed to get current directory")),
        "cd" => change_directory(parts.get(1)),
        "ls" => list_directory(parts.get(1)),
        "mkdir" => create_directory(parts.get(1)),
        "rm" => remove_file_or_directory(parts.get(1)),
        "touch" => create_file(parts.get(1)),
        _ => {
            let output = Command::new(parts[0]).args(&parts[1..]).output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8_lossy(&output.stdout).into_owned()
                    } else {
                        String::from_utf8_lossy(&output.stderr).into_owned()
                    }
                }
                Err(e) => format!("failed to exercute command: {}", e),
            }
        }
    }
}

fn print_banner() {
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
        "Welcome to the Batcave Terminal. Proceed with caution."
            .bright_yellow()
    );
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
