use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    io::{self, Error as IoError},
    path::Path,
    process::{Command, Stdio},
};

use colored::*;
use log::{error, info, LevelFilter};
use rustyline::{error::ReadlineError, Editor, Helper};

use simplelog::{CombinedLogger, Config, WriteLogger};

#[derive(Clone)]
struct Shell {
    env_vars: HashMap<String, String>,
    aliases: HashMap<String, String>,
    history: VecDeque<String>,
}

impl Shell {
    fn new() -> Self {
        let mut env_vars = HashMap::new();
        for (key, value) in env::vars() {
            env_vars.insert(key, value);
        }
        Shell {
            env_vars,
            aliases: HashMap::new(),
            history: VecDeque::with_capacity(1000),
        }
    }

    fn get_env(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    fn set_env(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }

    fn add_alias(&mut self, name: String, command: String) {
        self.aliases.insert(name, command);
    }

    fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    fn add_to_history(&mut self, command: String) {
        if self.history.len() >= 1000 {
            self.history.pop_front();
        }
        self.history.push_back(command);
    }

    fn _get_history(&self) -> &VecDeque<String> {
        &self.history
    }
}

fn main() -> io::Result<()> {
    setup_logging()?;
    print_banner();

    let mut shell = Shell::new();
    let helper = ShellHelper::new(&shell);
    let mut rl = Editor::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    rl.set_helper(Some(helper));

    shell.add_alias("ll".to_string(), "ls -la".to_string());
    shell.add_alias("cls".to_string(), "clear".to_string());

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

fn setup_logging() -> io::Result<()> {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|e| IoError::new(io::ErrorKind::NotFound, e))?;
    let log_path = Path::new(&home_dir).join(".batcave.log");

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        fs::File::create(log_path)?,
    )])
    .map_err(|e| IoError::new(io::ErrorKind::Other, e))?;

    Ok(())
}

fn autocomplete(input: &str, shell: &Shell) -> Vec<String> {
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

fn execute_command(command: &str, shell: &mut Shell) -> String {
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
        "Welcome to the Batcave Terminal. Proceed with caution.".bright_yellow()
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

fn expand_env_vars(command: &str, shell: &Shell) -> String {
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

fn tokenize_command(command: &str) -> Vec<String> {
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

// Add this struct and impl block for rustyline integration
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;

struct ShellHelper {
    shell: Shell,
}

impl ShellHelper {
    fn new(shell: &Shell) -> Self {
        ShellHelper {
            shell: shell.clone(),
        }
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let (start, word) = extract_word(line, pos);
        let completions = autocomplete(word, &self.shell);
        Ok((
            start,
            completions
                .into_iter()
                .map(|s| Pair {
                    display: s.clone(),
                    replacement: s,
                })
                .collect(),
        ))
    }
}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}
impl Highlighter for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
}

fn extract_word(line: &str, pos: usize) -> (usize, &str) {
    let start = line[..pos].rfind(char::is_whitespace).map_or(0, |i| i + 1);
    (start, &line[start..pos])
}
