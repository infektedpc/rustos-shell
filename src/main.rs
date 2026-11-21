//! Host binary: run the RustOS shell against the in-memory FS.
//! `rustos-shell` (interactive REPL) or `rustos-shell --eval "<cmd>"`.

use rustos_shell::{Shell, PROMPT};
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut shell = Shell::with_seeded_fs();

    // Non-interactive: one command.
    if args.len() >= 3 && args[1] == "--eval" {
        let out = shell.execute(&args[2]);
        print!("{}", out);
        return;
    }
    if args.len() >= 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("rustos-shell — RustOS userspace shell (host demo)");
        println!("usage: rustos-shell [--eval \"<command>\"]");
        println!("Seeded files: /hello.txt, /README.txt, /etc/motd, /etc/hostname");
        return;
    }

    println!("RustOS 0.1.0 — type 'help' for builtins, 'exit' to quit.");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("{}", PROMPT);
        let _ = stdout.flush();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let out = shell.execute(&line);
                print!("{}", out);
                let _ = stdout.flush();
                if !shell.running {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
