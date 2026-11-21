//! The RustOS shell: prompt, history, and builtins.
//!
//! Builtins: `help`, `echo`, `ls`, `cat`, `clear`, `uname`,
//! `history`, `reboot`, `exit`. Everything else is
//! "command not found". The shell is deliberately small: it parses
//! with the core-only parser and dispatches on the command name.

use crate::fs::InMemoryFs;
use crate::parser::parse_line;

pub const PROMPT: &str = "rustos> ";
pub const VERSION: &str = "RustOS 0.1.0";

pub struct Shell {
    pub fs: InMemoryFs,
    pub history: Vec<String>,
    /// False after `exit`; the REPL loop stops.
    pub running: bool,
    /// True after `reboot` (host demo: flag only, does not reboot).
    pub reboot_requested: bool,
}

impl Shell {
    pub fn new(fs: InMemoryFs) -> Self {
        Self {
            fs,
            history: Vec::new(),
            running: true,
            reboot_requested: false,
        }
    }

    pub fn with_seeded_fs() -> Self {
        Self::new(InMemoryFs::seeded())
    }

    /// Execute one input line. Returns the text to print (may be
    /// empty). Records non-empty lines in history.
    pub fn execute(&mut self, line: &str) -> String {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        self.history.push(trimmed.to_string());
        let cmd = parse_line(line);
        if cmd.is_empty() {
            return String::new();
        }
        let args: Vec<&str> = cmd.args[..cmd.argc].to_vec();
        match cmd.name {
            "help" => self.help(),
            "echo" => {
                if args.is_empty() {
                    "\n".to_string()
                } else {
                    format!("{}\n", args.join(" "))
                }
            }
            "ls" => self.ls(args.first().copied().unwrap_or("/")),
            "cat" => match args.first() {
                Some(path) => match self.fs.read(path) {
                    Some(contents) => {
                        if contents.ends_with('\n') {
                            contents.to_string()
                        } else {
                            format!("{}\n", contents)
                        }
                    }
                    None => format!("cat: {}: No such file or directory\n", path),
                },
                None => "usage: cat <path>\n".to_string(),
            },
            "clear" => "\x1b[2J\x1b[H".to_string(),
            "uname" => format!("{} x86_64 rustos\n", VERSION),
            "history" => {
                let mut out = String::new();
                for (i, h) in self.history.iter().enumerate() {
                    out.push_str(&format!("{:>4}  {}\n", i + 1, h));
                }
                out
            }
            "reboot" => {
                self.reboot_requested = true;
                "Rebooting...\n".to_string()
            }
            "exit" => {
                self.running = false;
                "Bye.\n".to_string()
            }
            other => format!("rustos: command not found: {}\n", other),
        }
    }

    fn help(&self) -> String {
        "RustOS shell — builtins:\n  \
         help              show this help\n  \
         echo <text...>    print text\n  \
         ls [path]         list a directory\n  \
         cat <path>        print a file\n  \
         clear             clear the screen\n  \
         uname             print system name\n  \
         history           show command history\n  \
         reboot            reboot the system\n  \
         exit              exit the shell\n"
            .to_string()
    }

    fn ls(&self, path: &str) -> String {
        match self.fs.list_dir(path) {
            Some(entries) => {
                if entries.is_empty() {
                    String::new()
                } else {
                    format!("{}\n", entries.join("  "))
                }
            }
            None => format!("ls: {}: No such file or directory\n", path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shell() -> Shell {
        Shell::with_seeded_fs()
    }

    #[test]
    fn builtins_echo_help_uname() {
        let mut sh = shell();
        assert_eq!(sh.execute("echo hello rustos"), "hello rustos\n");
        assert_eq!(sh.execute("echo"), "\n");
        assert!(sh.execute("help").contains("cat <path>"));
        assert!(sh.execute("uname").contains("RustOS 0.1.0"));
        assert_eq!(sh.execute("clear"), "\x1b[2J\x1b[H");
    }

    #[test]
    fn ls_and_cat() {
        let mut sh = shell();
        let ls = sh.execute("ls /");
        assert!(ls.contains("hello.txt"));
        assert!(ls.contains("etc/"));
        assert!(sh.execute("cat /hello.txt").contains("Hello from RustOS!"));
        assert!(sh.execute("cat /missing").contains("No such file"));
        assert!(sh.execute("cat").contains("usage: cat"));
        assert!(sh.execute("ls /missing").contains("No such file"));
    }

    #[test]
    fn unknown_history_exit_reboot() {
        let mut sh = shell();
        assert!(sh.execute("frobnicate").contains("command not found: frobnicate"));
        sh.execute("echo one");
        sh.execute("ls");
        let h = sh.execute("history");
        assert!(h.contains("echo one"));
        assert!(h.contains("ls"));
        assert!(!sh.reboot_requested);
        assert_eq!(sh.execute("reboot"), "Rebooting...\n");
        assert!(sh.reboot_requested);
        assert!(sh.running);
        assert_eq!(sh.execute("exit"), "Bye.\n");
        assert!(!sh.running);
        // Blank lines are not history and produce no output.
        let mut sh2 = shell();
        assert_eq!(sh2.execute("   "), "");
        assert!(sh2.history.is_empty());
    }
}
