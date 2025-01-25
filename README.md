# Live Die Repeat

A simple process manager that allows you to restart long-running commands using SIGUSR1 signals.

## Description

Live Die Repeat is a command-line utility that wraps around any shell command and provides the ability to restart it by sending a SIGUSR1 signal. This is particularly useful during development when you need to restart processes frequently.

## Installation

To build the project, you'll need Rust installed on your system. Then:

```bash
cargo install --path .
```

## Usage

Run any command through live_die_repeat:

```bash
live_die_repeat "your_command_here"
```

To restart the running command, send a SIGUSR1 signal to the process:

```bash
kill -SIGUSR1 <PID>
```

Or use `pkill`
```bash
pkill -USR1 live_die_repeat
```

The PID will be displayed when you start the program:
```
🎯 Process started with PID: 1234
👂 Signal handler thread started, waiting for SIGUSR1...
```

## Features

- Wraps any shell command
- Gracefully handles process termination
- Restarts processes on SIGUSR1 signal
- Preserves exit codes from child processes
- Simple and lightweight

## Dependencies

The project uses the following crates:
- `ctrlc`: Signal handling for Ctrl+C
- `signal-hook`: Unix signal handling

## Example

```bash
# Start a web server
live_die_repeat "python -m http.server 8000"

# In another terminal, restart the server:
pkill -USR1 live_die_repeat
```


## License

This project is open source and available under the MIT License.
