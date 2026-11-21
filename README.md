# rustos-shell

Userspace shell + init for RustOS.

- `src/parser.rs` — **core-only** (no `std`, no `alloc`): tokenizer /
  command parser. This module is written so the kernel can reuse it
  verbatim in a `no_std` build — it only slices the input `&str`.
- `src/syscall.rs` — syscall numbers the kernel will implement
  (`read`/`write`/`open`/`close`/`yield`/`exit`).
- `src/fs.rs` — a tiny in-memory filesystem (`InMemoryFs`) used by
  the host demo and the tests.
- `src/shell.rs` — the shell itself (builtins, history, prompt).
- `src/main.rs` — host binary `rustos-shell`: runs the shell against
  the in-memory FS on Linux so it can be demoed and tested.

## Build / test

```sh
cargo test
cargo build --release
```

Verify the parser really is core-only:

```sh
grep -rn "std::\|use std\|Vec<\|String" src/parser.rs || echo "parser.rs is core-only"
```

## Usage

```sh
# Interactive demo (seeded files: /hello.txt, /README.txt, /etc/motd)
./target/debug/rustos-shell
rustos> help
rustos> ls
rustos> cat /hello.txt
rustos> echo hello rustos
rustos> uname
rustos> history
rustos> exit

# One command, non-interactive
./target/debug/rustos-shell --eval "ls /"
```

## Builtins

`help`, `echo`, `ls [path]`, `cat <path>`, `clear`, `uname`,
`history`, `reboot`, `exit`. Prompt is `rustos> `. Unknown
commands print `rustos: command not found: <name>`.

## Limitations

- The host binary talks to an in-memory FS, not a real `rustfs`
  disk image — wiring it to a disk image is the kernel/FS step.
- `reboot` in the host demo only prints `Rebooting...` and sets a
  flag; it does not reboot the host (obviously).
- No pipes, redirection, or globbing yet — the parser tokenizes
  (with quotes) and the shell dispatches one builtin per line.
