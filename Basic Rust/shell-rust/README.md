# Rust Shell

A simple Rust-based shell that supports basic commands, including cd, exit, and piping (|) between commands. This shell allows users to execute system commands in a manner similar to most Unix-like command-line interfaces.

## Features

- *Execute System Commands*: Run standard system commands such as ls, grep, cat, and more.
- *Built-in Commands*:
  - cd <directory>: Change the current working directory.
  - exit: Exit the shell.
- *Command Piping*: Support for command piping using |, allowing you to connect the output of one command to the input of another (e.g., ls | grep rust).
  
## How to Use

1. *Clone the repository* (if applicable) or copy the source code into a Rust project.
2. *Build the project*:
    bash
    cargo build
    
3. *Run the shell*:
    bash
    cargo run
    

4. You will see a prompt `> ` where you can start typing commands.

### Example Commands

- Change directory:
    bash
    > cd /home/user
    

- List files and pipe the output to grep:
    bash
    > ls | grep rust
    

- Exit the shell:
    bash
    > exit
    

## Project Structure

- *main.rs*: Contains the shell's core functionality, including command handling, piping, and input/output management.

## Dependencies

The shell only uses Rust's standard library. No additional external dependencies are required.

## Error Handling

- If an invalid command is entered, the shell will print an error message.
- The cd command will print an error if the directory does not exist or is not accessible.

## PS
The project is based on [tutorial](https://www.joshmcguigan.com/blog/build-your-own-shell-rust/)