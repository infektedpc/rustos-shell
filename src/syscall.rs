//! Syscall numbers for the RustOS kernel.
//!
//! These are the numbers the kernel's syscall handler will dispatch
//! on. The shell (and later userspace programs) use them via the
//! `syscall` instruction / `int 0x80`-style trap the kernel defines.
//! Keep this file in sync with the kernel's dispatch table.

/// read(fd, buf, len) -> bytes read, or negative errno.
pub const SYS_READ: u64 = 0;
/// write(fd, buf, len) -> bytes written, or negative errno.
pub const SYS_WRITE: u64 = 1;
/// open(path, flags) -> fd, or negative errno.
pub const SYS_OPEN: u64 = 2;
/// close(fd) -> 0, or negative errno.
pub const SYS_CLOSE: u64 = 3;
/// yield() -> 0. Voluntarily give up the CPU.
pub const SYS_YIELD: u64 = 24;
/// exit(code) -> never returns.
pub const SYS_EXIT: u64 = 60;
/// uptime() -> milliseconds since boot. RustOS-private number 1000
/// (see `rustos-process/src/syscall.rs`): Linux has no uptime
/// syscall. Added in Batch 9 for the userspace shell's `uptime`
/// builtin.
pub const SYS_UPTIME: u64 = 1000;

/// Standard file descriptors.
pub const FD_STDIN: u64 = 0;
pub const FD_STDOUT: u64 = 1;
pub const FD_STDERR: u64 = 2;

/// Human-readable name for a syscall number, or "unknown".
pub fn syscall_name(n: u64) -> &'static str {
    match n {
        SYS_READ => "read",
        SYS_WRITE => "write",
        SYS_OPEN => "open",
        SYS_CLOSE => "close",
        SYS_YIELD => "yield",
        SYS_EXIT => "exit",
        SYS_UPTIME => "uptime",
        _ => "unknown",
    }
}

/// All defined syscalls, in number order.
pub const ALL_SYSCALLS: &[(u64, &str)] = &[
    (SYS_READ, "read"),
    (SYS_WRITE, "write"),
    (SYS_OPEN, "open"),
    (SYS_CLOSE, "close"),
    (SYS_YIELD, "yield"),
    (SYS_EXIT, "exit"),
    (SYS_UPTIME, "uptime"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_distinct_and_named() {
        let nums = [SYS_READ, SYS_WRITE, SYS_OPEN, SYS_CLOSE, SYS_YIELD, SYS_EXIT];
        for (i, a) in nums.iter().enumerate() {
            for b in nums.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
        assert_eq!(syscall_name(SYS_READ), "read");
        assert_eq!(syscall_name(SYS_WRITE), "write");
        assert_eq!(syscall_name(SYS_OPEN), "open");
        assert_eq!(syscall_name(SYS_EXIT), "exit");
        assert_eq!(syscall_name(SYS_YIELD), "yield");
        assert_eq!(syscall_name(SYS_UPTIME), "uptime");
        assert_eq!(SYS_UPTIME, 1000);
        assert_eq!(syscall_name(999), "unknown");
        assert_eq!(ALL_SYSCALLS.len(), 7);
        assert_eq!(FD_STDOUT, 1);
    }
}
