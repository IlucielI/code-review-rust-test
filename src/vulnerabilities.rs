use std::process::Command;
use std::ptr;

// Bug 1: Shell Command Injection
pub fn execute_user_script(script: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute");
    String::from_utf8_lossy(&output.stdout).to_string()
}

// Bug 2: Unsafe Raw Pointer Dereference without null check
pub unsafe fn read_from_raw_ptr(raw: *const u32) -> u32 {
    *raw // Segmentation fault / memory corruption if pointer is null/dangling
}

// Safe Guard: Safe slice indexing (MUST NOT be flagged)
pub fn read_safe(data: &[u32], idx: usize) -> Option<u32> {
    data.get(idx).copied()
}
