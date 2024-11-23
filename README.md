# Rust terminal notepad

A terminal-based text editor written in Rust, using the `crossterm` crate for handling terminal interactions. This program allows users to input text, navigate with the cursor, and save their work to a file.

## Features

- Insert text interactively in the terminal.
- Use arrow keys to navigate.
- Save your text to a file by pressing `Ctrl+C`.
- Exit the program by pressing `Esc`.

---

## Prerequisites

1. **Rust Toolchain**  
   Ensure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **crossterm Crate**  
   The program depends on the `crossterm` crate. This will be handled by `cargo` during the build process.

---

## Building the Program

1. **Clone the Repository**  
   Clone the repository to your local machine:

   ```sh
   git@github.com:Posadowski/rust_terminal_notepad.git
   cd rust_terminal_notepad
   ```

2. **Build the Program**  
   Use `cargo` to build the project:

   ```sh
   cargo build 
   ```

   The compiled executable will be located in the `target/release` directory.

3. **Run the Program**  
   Execute the program with:

   ```sh
   cargo run
   ```

---

## How to Use

### Running the Program
Once the program is running, you will be placed in a blank terminal where you can start typing. Use the following commands to interact with the program:

### Key Bindings

| Key                  | Action                         |
|----------------------|--------------------------------|
| Any character        | Inserts the character at the cursor position. |
| `Backspace`          | Deletes the character before the cursor.      |
| `Enter`              | Inserts a newline.                         |
| `Ctrl+C`             | Saves the text to a file and exits the program. |
| `Esc`                | Exits the program without saving.            |

### Saving the File
- By default, the program saves your input to a file named `untitled.txt` in the current directory when you press `Ctrl+C`.

### Exiting
- Press `Esc` to exit the program without saving your input.

---

## Example

```sh
$ cargo run
# Start typing in the terminal:
Hello, World!
This is a simple text editor.

# Press Ctrl+C to save:
File written to untitled.txt
$ cat output.txt
Hello, World!
This is a simple text editor.
```

---

## Troubleshooting

### Double Characters or Unexpected Behavior
If you encounter doubled characters or unusual terminal behavior, ensure:
1. You are using a compatible terminal (e.g., Linux `bash`, macOS `zsh`, or Windows `PowerShell`).
2. The terminal is in raw mode and not being interfered with by another program.

### Resetting the Terminal
If your terminal is not restored to its normal state after an error, run:

```sh
reset
```

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

Enjoy your Rust terminal notepad experience! 🎉
