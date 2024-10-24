use colored::*;

const LOGO: &str = r#"
   🦇 Batcave Shell 🦇
      ___      _    
     | _ )__ _| |_  
     | _ / _` |  _| 
     |___\__,_|\__| 
                    
"#;

pub fn handle_help(args: &[String]) -> String {
    if args.is_empty() {
        general_help()
    } else {
        command_specific_help(&args[0])
    }
}

fn general_help() -> String {
    let mut help = String::new();

    // Add ASCII art logo
    help.push_str(&format!("{}\n", LOGO.bright_purple()));

    help.push_str(&format!(
        "{}\n",
        "Available Commands".bright_yellow().bold()
    ));
    help.push_str(&format!("{}\n\n", "==================".bright_yellow()));

    help.push_str(&format!(
        "\n{}\n",
        "Directory Navigation:".bright_blue().bold()
    ));
    help.push_str(&format!(
        "  {}    - Push directory to stack\n",
        "pushd".green()
    ));
    help.push_str(&format!(
        "  {}     - Pop directory from stack\n",
        "popd".green()
    ));

    // Process Management
    help.push_str(&format!(
        "\n{}\n",
        "Process Management:".bright_blue().bold()
    ));
    help.push_str(&format!("  {}     - List active jobs\n", "jobs".green()));

    help.push_str(&format!(
        "\n{}\n",
        "History Management:".bright_blue().bold()
    ));
    help.push_str(&format!(
        "  {}  - Show command history\n",
        "history".green()
    ));

    help.push_str(&format!("{}\n", "File Operations:".bright_blue().bold()));
    help.push_str(&format!(
        "  {}      - List directory contents\n",
        "ls".green()
    ));
    help.push_str(&format!(
        "  {}      - Print working directory\n",
        "pwd".green()
    ));
    help.push_str(&format!("  {}      - Change directory\n", "cd".green()));
    help.push_str(&format!("  {}    - Create directory\n", "mkdir".green()));
    help.push_str(&format!(
        "  {}      - Remove file/directory\n",
        "rm".green()
    ));
    help.push_str(&format!("  {}    - Create empty file\n", "touch".green()));

    help.push_str(&format!("\n{}\n", "Shell Management:".bright_blue().bold()));
    help.push_str(&format!(
        "  {}    - Create .batcaverc config\n",
        "init".green()
    ));
    help.push_str(&format!(
        "  {}    - Set as default shell\n",
        "set-default".green()
    ));
    help.push_str(&format!(
        "  {} - Remove as default shell\n",
        "remove-default".green()
    ));
    help.push_str(&format!("  {}    - Exit the shell\n", "exit".green()));

    help.push_str(&format!(
        "\n{}\n",
        "Environment & Aliases:".bright_blue().bold()
    ));
    help.push_str(&format!(
        "  {}    - Define/display aliases\n",
        "alias".green()
    ));
    help.push_str(&format!(
        "  {}   - Set environment variables\n",
        "export".green()
    ));
    help.push_str(&format!(
        "  {}      - Show environment variables\n",
        "env".green()
    ));

    help.push_str(&format!("\n{}\n", "System & Help:".bright_blue().bold()));
    help.push_str(&format!(
        "  {}     - Show system information\n",
        "info".green()
    ));
    help.push_str(&format!(
        "  {}     - Display this help message\n",
        "help".green()
    ));
    help.push_str(&format!(
        "  {}     - Show command echo output\n",
        "echo".green()
    ));

    help.push_str(&format!("\n{}\n", "Shell Features:".bright_blue().bold()));
    help.push_str(" • Command history (↑/↓ arrows)\n");
    help.push_str(" • Tab completion for commands & files\n");
    help.push_str(" • Environment variable expansion ($VAR)\n");
    help.push_str(" • Custom aliases and configurations\n");

    help.push_str(&format!("\n{}: ", "Usage".bright_yellow()));
    help.push_str("help <command> for specific command details\n");

    help
}

fn command_specific_help(command: &str) -> String {
    let help_text = match command {
        "pushd" => format!(
            "{}\n{}\n\n{}\n  pushd /path/to/dir",
            "pushd <directory>".bright_yellow().bold(),
            "Push current directory to stack and change to new directory".bright_blue(),
            "Example:".bright_green()
        ),

        "popd" => format!(
            "{}\n{}\n\n{}\n  popd",
            "popd".bright_yellow().bold(),
            "Pop directory from stack and change to it".bright_blue(),
            "Example:".bright_green()
        ),

        "jobs" => format!(
            "{}\n{}\n\n{}\n  jobs",
            "jobs".bright_yellow().bold(),
            "List currently running background jobs".bright_blue(),
            "Example:".bright_green()
        ),

        "history" => format!(
            "{}\n{}\n\n{}\n  history",
            "history".bright_yellow().bold(),
            "Display command history".bright_blue(),
            "Example:".bright_green()
        ),

        "systeminfo" => format!(
            "{}\n{}\n\n{}\n  systeminfo on\n  systeminfo off\n  systeminfo status",
            "systeminfo [on|off|status]".bright_yellow().bold(),
            "Configure system information display on startup".bright_blue(),
            "Examples:".bright_green()
        ),

        "echo" => format!(
            "{}\n{}\n\n{}\n  echo Hello, World!\n  echo $USER is using Batcave",
            "echo [text...]".bright_yellow().bold(),
            "Display text or variable content".bright_blue(),
            "Examples:".bright_green()
        ),

        "pwd" => format!(
            "{}\n{}\n\n{}\n  pwd",
            "pwd".bright_yellow().bold(),
            "Print current working directory path".bright_blue(),
            "Example:".bright_green()
        ),

        "cd" => format!(
            "{}\n{}\n\n{}\n  cd /home/user\n  cd ..\n  cd ~",
            "cd [directory]".bright_yellow().bold(),
            "Change current directory".bright_blue(),
            "Examples:".bright_green()
        ),

        "ls" => format!(
            "{}\n{}\n\n{}\n  ls\n  ls /home\n  ls -la",
            "ls [directory]".bright_yellow().bold(),
            "List directory contents".bright_blue(),
            "Examples:".bright_green()
        ),

        "mkdir" => format!(
            "{}\n{}\n\n{}\n  mkdir new_folder\n  mkdir -p parent/child",
            "mkdir <directory>".bright_yellow().bold(),
            "Create new directory".bright_blue(),
            "Examples:".bright_green()
        ),

        "rm" => format!(
            "{}\n{}\n\n{}\n  rm file.txt\n  rm -r directory",
            "rm <path>".bright_yellow().bold(),
            "Remove file or directory".bright_blue(),
            "Examples:".bright_green()
        ),

        "touch" => format!(
            "{}\n{}\n\n{}\n  touch newfile.txt\n  touch file1.txt file2.txt",
            "touch <filename>".bright_yellow().bold(),
            "Create empty file or update timestamp".bright_blue(),
            "Examples:".bright_green()
        ),

        "alias" => format!(
            "{}\n{}\n\n{}\n  alias ll='ls -la'\n  alias",
            "alias [name=value]".bright_yellow().bold(),
            "Create command aliases or show existing ones".bright_blue(),
            "Examples:".bright_green()
        ),

        "export" => format!(
            "{}\n{}\n\n{}\n  export PATH=$PATH:/new/path\n  export EDITOR=vim",
            "export NAME=value".bright_yellow().bold(),
            "Set environment variables".bright_blue(),
            "Examples:".bright_green()
        ),

        "env" => format!(
            "{}\n{}\n\n{}\n  env",
            "env".bright_yellow().bold(),
            "Display all environment variables".bright_blue(),
            "Example:".bright_green()
        ),

        "init" => format!(
            "{}\n{}\n\n{}\n  init",
            "init".bright_yellow().bold(),
            "Create default .batcaverc configuration file".bright_blue(),
            "Example:".bright_green()
        ),

        "set-default" => format!(
            "{}\n{}\n\n{}\n  set-default",
            "set-default".bright_yellow().bold(),
            "Set Batcave as your default shell".bright_blue(),
            "Example:".bright_green()
        ),

        "remove-default" => format!(
            "{}\n{}\n\n{}\n  remove-default",
            "remove-default".bright_yellow().bold(),
            "Remove Batcave as default shell (revert to bash)".bright_blue(),
            "Example:".bright_green()
        ),

        "info" => format!(
            "{}\n{}\n\n{}\n  info",
            "info".bright_yellow().bold(),
            "Display system information and resources".bright_blue(),
            "Example:".bright_green()
        ),

        "help" => format!(
            "{}\n{}\n\n{}\n  help\n  help cd",
            "help [command]".bright_yellow().bold(),
            "Display help information".bright_blue(),
            "Examples:".bright_green()
        ),

        "exit" => format!(
            "{}\n{}\n\n{}\n  exit",
            "exit".bright_yellow().bold(),
            "Exit the Batcave shell".bright_blue(),
            "Example:".bright_green()
        ),

        _ => format!(
            "No help available for '{}'\nType 'help' for a list of commands.",
            command
        )
        .red()
        .to_string(),
    };

    help_text
}
