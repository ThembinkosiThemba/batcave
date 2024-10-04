# Batcave Terminal

<p align="start">
    <img src="./src/image.png" alt="genesis" />
</p>


Welcome to the Batcave Terminal, a mysterious and powerful command-line interface inspired by the Dark Knight's lair. This Rust-based terminal emulator provides a unique, Batman-themed experience while offering essential file system navigation and manipulation capabilities.

## Features

- 🦇 Batman-inspired ASCII art banner
- 🌑 Dark, mysterious theme with colored output
- 📁 Basic file system operations (cd, ls, mkdir, rm, touch)
- 📝 Command logging for auditing and debugging
- 🔍 Error handling and informative messages

## Installation

To install and run the Batcave Terminal, follow these steps:

You can install it from `cargo` buy tunning

```sh
cargo install batcave
```

or by cloning the repo and running

Clone the repository:

```sh
git clone https://github.com/ThembinkosiThemba/batcave.git
cd batcave
cargo build --release
cargo run --release
```

## Usage

Once you've launched the Batcave Terminal, you'll be greeted with the Batman-inspired ASCII art and a prompt. Here are the available commands:

- `cd [directory]`: Change the current directory
- `pwd`: Print the current working directory
- `ls [directory]`: List the contents of a directory
- `mkdir [directory_name]`: Create a new directory
- `rm [file_or_directory]`: Remove a file or directory
- `touch [file_name]`: Create a new file
- `echo [message]`: Print a message to the terminal
- `exit`: Exit the Batcave Terminal

Any other commands will be passed to the system shell for execution.

## Customization

Feel free to modify the ASCII art, colors, or add new commands to make the Batcave Terminal your own. The main logic is contained in `src/main.rs`.

## Contributing

Contributions to the Batcave Terminal are welcome! Please feel free to submit pull requests, report bugs, or suggest new features.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the Batman universe
- Built with Rust and various helpful crates

Remember, with great power comes great responsibility. Use the Batcave Terminal wisely, and may it serve you well in your coding adventures!
