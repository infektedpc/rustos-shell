//! A tiny in-memory filesystem for the host shell demo and tests.
//!
//! Paths are absolute (`/hello.txt`, `/etc/motd`). Directories are
//! implicit: a path's parent is a directory if any file lives under
//! it. This is a stand-in for the on-disk `rustfs` the kernel will
//! mount; the shell only depends on the small API below, so swapping
//! in a real FS means implementing the same methods.

use std::collections::BTreeMap;

/// In-memory file store: path -> contents.
#[derive(Debug, Default, Clone)]
pub struct InMemoryFs {
    files: BTreeMap<String, String>,
}

impl InMemoryFs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed the demo filesystem the host binary ships with.
    pub fn seeded() -> Self {
        let mut fs = Self::new();
        fs.write("/hello.txt", "Hello from RustOS!\n");
        fs.write(
            "/README.txt",
            "RustOS 0.1.0\nA tiny operating system written in Rust.\nTry: help, ls, cat /hello.txt, uname\n",
        );
        fs.write("/etc/motd", "Welcome to RustOS.\n");
        fs.write("/etc/hostname", "rustos\n");
        fs
    }

    pub fn write(&mut self, path: &str, contents: &str) {
        self.files.insert(normalize(path), contents.to_string());
    }

    pub fn read(&self, path: &str) -> Option<&str> {
        self.files.get(&normalize(path)).map(|s| s.as_str())
    }

    pub fn exists(&self, path: &str) -> bool {
        self.files.contains_key(&normalize(path)) || self.is_dir(path)
    }

    pub fn is_dir(&self, path: &str) -> bool {
        let dir = normalize(path);
        if dir == "/" {
            return true;
        }
        let prefix = format!("{}/", dir.trim_end_matches('/'));
        self.files.keys().any(|k| k.starts_with(&prefix))
    }

    /// Names directly under `path` (files and subdirectories),
    /// sorted. Returns `None` if the path is not a directory.
    pub fn list_dir(&self, path: &str) -> Option<Vec<String>> {
        if !self.is_dir(path) {
            return None;
        }
        let dir = normalize(path);
        let prefix = if dir == "/" {
            "/".to_string()
        } else {
            format!("{}/", dir.trim_end_matches('/'))
        };
        let mut out: Vec<String> = Vec::new();
        for key in self.files.keys() {
            if let Some(rest) = key.strip_prefix(&prefix) {
                if rest.is_empty() {
                    continue;
                }
                let first = rest.split('/').next().unwrap_or(rest);
                let entry = if rest.contains('/') {
                    format!("{}/", first)
                } else {
                    first.to_string()
                };
                if !out.contains(&entry) {
                    out.push(entry);
                }
            }
        }
        out.sort();
        Some(out)
    }
}

fn normalize(path: &str) -> String {
    let p = path.trim();
    if p.is_empty() {
        return "/".to_string();
    }
    let mut out = String::new();
    for part in p.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            // Drop the last component, if any.
            if let Some(idx) = out.rfind('/') {
                out.truncate(idx);
            }
            continue;
        }
        out.push('/');
        out.push_str(part);
    }
    if out.is_empty() {
        "/".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_files_and_listing() {
        let fs = InMemoryFs::seeded();
        assert_eq!(fs.read("/hello.txt"), Some("Hello from RustOS!\n"));
        assert!(fs.read("/nope.txt").is_none());
        assert!(fs.is_dir("/"));
        assert!(fs.is_dir("/etc"));
        assert!(!fs.is_dir("/hello.txt"));
        let root = fs.list_dir("/").unwrap();
        assert!(root.contains(&"hello.txt".to_string()));
        assert!(root.contains(&"etc/".to_string()));
        assert_eq!(fs.list_dir("/etc").unwrap(), vec!["hostname".to_string(), "motd".to_string()]);
        assert!(fs.list_dir("/hello.txt").is_none());
        let mut fs2 = InMemoryFs::new();
        fs2.write("a/b.txt", "x");
        assert_eq!(fs2.read("/a/b.txt"), Some("x"));
    }
}
