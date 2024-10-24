use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    path::Path,
    time::SystemTime,
};

use colored::*;

use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Context, Helper,
};

use crate::utils::autocomplete;

/// This Shell struct is the core data structure that maintains the state of the shell session.
#[derive(Clone)]
pub struct Shell {
    pub env_vars: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub history: VecDeque<String>,
    command_start_time: Option<SystemTime>,
    dir_stack: Vec<String>,
}

impl Shell {
    // creates a new shell instance with the current environment variables
    pub fn new() -> Self {
        let mut shell = Shell {
            env_vars: HashMap::new(),
            aliases: HashMap::new(),
            history: VecDeque::with_capacity(1000),
            command_start_time: None,
            dir_stack: Vec::new(),
        };

        for (key, value) in env::vars() {
            shell.env_vars.insert(key, value);
        }

        shell.ensure_config_exists();

        shell.load_config();

        shell
    }

    pub fn start_command_timer(&mut self) {
        self.command_start_time = Some(SystemTime::now());
    }

    pub fn end_command_timer(&mut self) -> Option<f64> {
        self.command_start_time
            .and_then(|start| start.elapsed().ok().map(|duration| duration.as_secs_f64()))
    }

    pub fn get_show_system_info(&self) -> bool {
        self.get_env("SHOW_SYSTEM_INFO")
            .map(|val| val.to_lowercase() == "true")
            .unwrap_or(true)
    }

    // Pushes current directory to stack
    // Used for directory navigation
    pub fn push_dir(&mut self, dir: String) {
        self.dir_stack.push(dir)
    }

    // Retrieves previous directory
    // Used for directory navigation
    pub fn pop_dir(&mut self) -> Option<String> {
        self.dir_stack.pop()
    }

    // Add command suggestions
    // pub fn _suggest_command(&self, failed_command: &str) -> Option<String> {
    //     let known_commands = ["ls", "cd", "pwd", "mkdir", "rm", "touch", "help"];
    //     let matcher = SkimMatcherV2::default();

    //     known_commands
    //         .iter()
    //         .filter_map(|&cmd| {
    //             matcher
    //                 .fuzzy_match(cmd, failed_command)
    //                 .map(|score| (cmd, score))
    //         })
    //         .max_by_key(|&(_, score)| score)
    //         .map(|(cmd, _)| cmd.to_string())
    // }

    pub fn set_show_system_info(&mut self, show: bool) {
        self.set_env("SHOW_SYSTEM_INFO".to_string(), show.to_string());
        if let Ok(home) = env::var("HOME") {
            let config_path = format!("{}/.batcaverc", home);
            if let Ok(content) = fs::read_to_string(&config_path) {
                let mut new_lines = Vec::new();
                let mut found = false;

                for line in content.lines() {
                    if line.trim().starts_with("export SHOW_SYSTEM_INFO=") {
                        new_lines.push(format!("export SHOW_SYSTEM_INFO=\"{}\"", show));
                        found = true;
                    } else {
                        new_lines.push(line.to_string());
                    }
                }

                if !found {
                    new_lines.push(format!("export SHOW_SYSTEM_INFO=\"{}\"", show));
                }

                let new_content = new_lines.join("\n");
                let _ = fs::write(&config_path, new_content);
            }
        }
    }

    fn load_config(&mut self) {
        if let Ok(home) = env::var("HOME") {
            let config_path = format!("{}/.batcaverc", home);
            if let Ok(content) = fs::read_to_string(&config_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with('#') || line.is_empty() {
                        continue;
                    }

                    if line.starts_with("alias ") {
                        if let Some(alias_def) = line.strip_prefix("alias ") {
                            if let Some((name, command)) = alias_def.split_once('=') {
                                let name = name.trim().to_string();
                                let command = command.trim().trim_matches('"').to_string();
                                self.add_alias(name, command);
                            }
                        }
                    } else if line.starts_with("export ") {
                        if let Some(export_def) = line.strip_prefix("export ") {
                            if let Some((name, value)) = export_def.split_once('=') {
                                let name = name.trim().to_string();
                                let value = value.trim().trim_matches('"').to_string();
                                self.set_env(name, value);
                            }
                        }
                    }
                }
            }
        }
    }

    fn ensure_config_exists(&self) {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
        let config_path = format!("{}/.batcaverc", home);

        if !Path::new(&config_path).exists() {
            let config_content = r#"# Batcave Shell Configuration

# Default aliases
alias ll="ls -la"
alias cls="clear"
alias gst="git status"
alias gco="git checkout"

# Environment variables
export PATH="$HOME/.cargo/bin:$PATH"
export EDITOR="vim"
export TERM="xterm-256color"

# Custom prompt settings
export PS1="🦇 \w> "

# Add your custom configurations below
"#;

            if let Ok(_) = fs::write(&config_path, config_content) {
                println!(
                    "{}",
                    format!("Initialized configuration at {}", config_path).green()
                );
            } else {
                eprintln!(
                    "{}",
                    format!("Failed to create configuration at {}", config_path).red()
                );
            }
        }
    }

    // function to retrieve an environment variable
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    // function to set an environment variabl
    pub fn set_env(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }

    // function to add a command alias
    pub fn add_alias(&mut self, name: String, command: String) {
        self.aliases.insert(name, command);
    }

    // function to get a command alias
    pub fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    // function to add a command to history
    pub fn add_to_history(&mut self, command: String) {
        if self.history.len() >= 1000 {
            self.history.pop_front();
        }
        self.history.push_back(command);
    }

    // retrieves a command from history
    fn _get_history(&self) -> &VecDeque<String> {
        &self.history
    }
}

// The ShellHelper struct is used for rustyline integration,
// providing auto-completion and other line editing features.
pub struct ShellHelper {
    shell: Shell,
}

impl ShellHelper {
    pub fn new(shell: &Shell) -> Self {
        ShellHelper {
            shell: shell.clone(),
        }
    }
}

// Complete provides auto-completion functionality
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
