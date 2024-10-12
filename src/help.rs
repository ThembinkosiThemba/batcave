use colored::*;

pub fn handle_help(args: &[String]) -> String {
    if args.is_empty() {
        general_help()
    } else {
        command_specific_help(&args[0])
    }
}


fn general_help() -> String {
    let mut help = String::new();
    help.push_str(&format!("{}\n\n", "Batcave Shell Help".bright_yellow().bold()));
    help.push_str("Available commands:\n");
    help.push_str(&format!("  {}     - Display a line of text\n", "echo".green()));
    help.push_str(&format!("  {}      - Print the current working directory\n", "pwd".green()));
    help.push_str(&format!("  {}       - Change the current directory\n", "cd".green()));
    help.push_str(&format!("  {}       - List directory contents\n", "ls".green()));
    help.push_str(&format!("  {}    - Create a new directory\n", "mkdir".green()));
    help.push_str(&format!("  {}       - Remove files or directories\n", "rm".green()));
    help.push_str(&format!("  {}    - Create a new file\n", "touch".green()));
    help.push_str(&format!("  {}    - Define or display aliases\n", "alias".green()));
    help.push_str(&format!("  {}   - Set environment variables\n", "export".green()));
    help.push_str(&format!("  {}      - Display environment variables\n", "env".green()));
    help.push_str(&format!("  {}     - Display system information\n", "info".green()));
    help.push_str(&format!("  {}     - Display this help message or command-specific help\n", "help".green()));
    help.push_str(&format!("  {}     - Exit the Batcave Shell\n\n", "exit".green()));
    help.push_str("For more information on a specific command, type 'help <command>'.\n");
    help.push_str("\nAdditional Features:\n");
    help.push_str("- Command history (use up/down arrows)\n");
    help.push_str("- Tab completion for commands, aliases, env variables, and files\n");
    help.push_str("- Environment variable expansion (e.g., $HOME)\n");
    help
}

fn command_specific_help(command: &str) -> String {
    match command {
        "echo" => "echo [text...]\n  Display a line of text.\n  Example: echo Hello, Batcave!".to_string(),
        "pwd" => "pwd\n  Print the current working directory.".to_string(),
        "cd" => "cd [directory]\n  Change the current directory.\n  Example: cd /home/batman".to_string(),
        "ls" => "ls [directory]\n  List directory contents.\n  Example: ls /batcave/gadgets".to_string(),
        "mkdir" => "mkdir <directory>\n  Create a new directory.\n  Example: mkdir secret_plans".to_string(),
        "rm" => "rm <file/directory>\n  Remove files or directories.\n  Example: rm old_plans.txt".to_string(),
        "touch" => "touch <filename>\n  Create a new file.\n  Example: touch new_gadget.txt".to_string(),
        "alias" => "alias [name[=value]]\n  Define or display aliases.\n  Example: alias ll='ls -la'".to_string(),
        "export" => "export NAME=value\n  Set environment variables.\n  Example: export BATCAVE_LOCATION=/secret/cave".to_string(),
        "env" => "env\n  Display all environment variables.".to_string(),
        "info" => "info\n  Display system information including CPU and memory usage.".to_string(),
        "help" => "help [command]\n  Display help information for Batcave Shell or a specific command.".to_string(),
        "exit" => "exit\n  Exit the Batcave Shell.".to_string(),
        _ => format!("No help available for '{}'", command),
    }
}