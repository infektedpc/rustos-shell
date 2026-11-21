//! Core-only command-line parser.
//!
//! This module uses **no `std` and no `alloc`**: it only borrows
//! slices of the input line. It is written so the RustOS kernel can
//! reuse it verbatim in a `no_std` build.
//!
//! Grammar: whitespace-separated words; single- and double-quoted
//! words may contain spaces; `#` starts a comment (outside quotes);
//! at most 8 arguments (plus the command name) are kept — extras
//! are counted in `truncated`.

/// A parsed command line. All strings borrow from the input line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedCommand<'a> {
    /// Command name (first word), or `""` for a blank line.
    pub name: &'a str,
    /// Arguments after the name (up to 8).
    pub args: [&'a str; 8],
    /// Number of arguments actually stored in `args`.
    pub argc: usize,
    /// True if the line had more than 8 arguments.
    pub truncated: bool,
}

impl<'a> ParsedCommand<'a> {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
    /// All words: name followed by args.
    pub fn words(&self) -> [&'a str; 9] {
        let mut w = [""; 9];
        w[0] = self.name;
        let mut i = 0;
        while i < self.argc {
            w[i + 1] = self.args[i];
            i += 1;
        }
        w
    }
}

const MAX_ARGS: usize = 8;
const MAX_TOKENS: usize = MAX_ARGS + 1;

/// Parse one line into a `ParsedCommand`.
///
/// Returns an empty command (name == "") for blank lines and
/// comment-only lines.
pub fn parse_line(line: &str) -> ParsedCommand<'_> {
    let bytes = line.as_bytes();
    let mut tokens: [Range; MAX_TOKENS] = [Range { start: 0, end: 0 }; MAX_TOKENS];
    let mut ntokens = 0usize;
    let mut truncated = false;
    let mut i = 0usize;
    let n = bytes.len();
    while i < n {
        // Skip whitespace.
        while i < n && is_space(bytes[i]) {
            i += 1;
        }
        if i >= n {
            break;
        }
        if bytes[i] == b'#' {
            break; // comment to end of line
        }
        // One token.
        let start;
        let end;
        if bytes[i] == b'\'' || bytes[i] == b'"' {
            let quote = bytes[i];
            i += 1;
            start = i;
            while i < n && bytes[i] != quote {
                i += 1;
            }
            end = i;
            if i < n {
                i += 1; // closing quote
            }
        } else {
            start = i;
            while i < n && !is_space(bytes[i]) && bytes[i] != b'#' {
                i += 1;
            }
            end = i;
        }
        if ntokens < MAX_TOKENS {
            tokens[ntokens] = Range { start, end };
            ntokens += 1;
        } else {
            truncated = true;
        }
    }
    let mut args = [""; MAX_ARGS];
    let mut argc = 0usize;
    let mut idx = 1usize;
    while idx < ntokens {
        args[argc] = &line[tokens[idx].start..tokens[idx].end];
        argc += 1;
        idx += 1;
    }
    let name = if ntokens == 0 {
        ""
    } else {
        &line[tokens[0].start..tokens[0].end]
    };
    ParsedCommand {
        name,
        args,
        argc,
        truncated,
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

fn is_space(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\r' || b == b'\n'
}

/// Count the words in a line (quotes respected), core-only helper.
pub fn count_tokens(line: &str) -> usize {
    let c = parse_line(line);
    if c.is_empty() {
        0
    } else {
        c.argc + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_words() {
        let c = parse_line("echo hello world");
        assert_eq!(c.name, "echo");
        assert_eq!(c.argc, 2);
        assert_eq!(c.args[0], "hello");
        assert_eq!(c.args[1], "world");
        assert!(!c.truncated);
    }

    #[test]
    fn blank_and_comment_lines() {
        assert!(parse_line("").is_empty());
        assert!(parse_line("   \t ").is_empty());
        assert!(parse_line("# just a comment").is_empty());
        let c = parse_line("echo hi # trailing comment");
        assert_eq!(c.name, "echo");
        assert_eq!(c.argc, 1);
    }

    #[test]
    fn quoted_words_keep_spaces() {
        let c = parse_line("echo 'hello world' \"and you\"");
        assert_eq!(c.argc, 2);
        assert_eq!(c.args[0], "hello world");
        assert_eq!(c.args[1], "and you");
    }

    #[test]
    fn extra_whitespace_and_truncation() {
        let c = parse_line("  ls   -l\t/foo  ");
        assert_eq!(c.name, "ls");
        assert_eq!(c.args[0], "-l");
        let many = parse_line("cmd a b c d e f g h i j k");
        assert_eq!(many.argc, 8);
        assert!(many.truncated);
        assert_eq!(count_tokens("echo a b"), 3);
        assert_eq!(count_tokens(""), 0);
    }
}
