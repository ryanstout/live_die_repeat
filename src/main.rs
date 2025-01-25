use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::env;
use signal_hook::consts::signal::SIGUSR1;
use signal_hook::iterator::Signals;
use std::thread;

fn main() {
    // Get command line arguments, skipping the program name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: live_die_repeat <command>");
        std::process::exit(1);
    }

    // Use the first argument as the complete command
    let command = &args[0];

    // Create shared flag for signal handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up SIGUSR1 handler
    let mut signals = Signals::new(&[SIGUSR1]).expect("Error setting up signal handler");
    println!("🎯 Process started with PID: {}", std::process::id());
    thread::spawn(move || {
        println!("👂 Signal handler thread started, waiting for SIGUSR1...");
        for _ in signals.forever() {
            println!("\n🔄 Received SIGUSR1 - Restarting command...");
            r.store(false, Ordering::SeqCst);
        }
    });

    // Main loop
    let mut child: Option<Child>;
    loop {
        // Reset the running flag
        running.store(true, Ordering::SeqCst);

        // Start the command in a shell
        child = Some(Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Failed to execute command"));

        // Wait for either the process to finish or a signal
        while running.load(Ordering::SeqCst) {
            if let Some(child) = &mut child {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("Process exited with status: {}", status);
                        // Exit with the same status code as the child process
                        std::process::exit(status.code().unwrap_or(1));
                    }
                    Ok(None) => {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => {
                        eprintln!("Error waiting for process: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }

        // Kill the process if it's still running
        if let Some(mut c) = child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
} 