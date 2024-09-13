use std::env; // For changing the current working directory with cd
use std::io::{stdin, stdout, Write}; // For handling user input and output
use std::path::Path; // For working with filesystem paths
use std::process::{Child, Command, Stdio}; // For spawning commands and handling I/O streams

fn main() {
    loop {
        // Print the shell prompt and flush the output to ensure it appears
        print!("> ");
        stdout().flush().unwrap(); // Ensure prompt is shown before waiting for input

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap(); // Read user input

        // Split the input into separate commands based on the pipe symbol (|) and make it peekable
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        // Process each command in the pipeline
        while let Some(command) = commands.next() {
            // Split the command into the command name and its arguments
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            // Handle built-in commands like cd and exit
            match command {
                "cd" => {
                    // Change directory to the provided argument or default to / if none is provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        // Print error if changing directory fails
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                }
                "exit" => return, // Exit the shell
                command => {
                    // Set up input from the previous command (if piped), otherwise inherit from stdin
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    // Set up output for the current command, pipe it if there are more commands
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped() // Pipe output to the next command
                    } else {
                        Stdio::inherit() // Send output to the shell's stdout
                    };

                    // Spawn the command with the specified input and output
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // Check if the command executed successfully
                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e); // Print the error if the command fails
                        }
                    };
                }
            }
        }

        // Wait for the last command in the pipeline to finish before prompting for the next input
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
