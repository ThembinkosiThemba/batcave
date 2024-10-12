use std::{
    collections::{HashMap, VecDeque},
    env,
};

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
}

impl Shell {
    // creates a new shell instance with the current environment variables
    pub fn new() -> Self {
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
