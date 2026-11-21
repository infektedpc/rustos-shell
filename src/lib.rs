//! rustos-shell: userspace shell + init for RustOS.
//!
//! * `parser` — core-only (no `std`, no `alloc`), reusable `no_std`.
//! * `syscall` — syscall numbers the kernel will implement.
//! * `fs` — in-memory filesystem for the host demo / tests.
//! * `shell` — builtins, history, prompt.

pub mod fs;
pub mod parser;
pub mod shell;
pub mod syscall;

pub use shell::{Shell, PROMPT, VERSION};
