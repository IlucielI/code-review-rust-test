# Rust Benchmark Test Suite

[![Rust Version](https://img.shields.io/badge/Rust-1.75%2B-DEA584.svg?logo=rust)](https://www.rust-lang.org/)
[![Benchmark Category](https://img.shields.io/badge/Benchmark-Security%2C%20Unsafe%20%26%20Panics-red.svg)](#test-case-matrix)
[![Safe Guard](https://img.shields.io/badge/False%20Positive%20Guard-Active-brightgreen.svg)](#anti-false-positive-guard-file)

Benchmark test suite for automated code review engines on Rust systems and web services. This repository evaluates review engines on catching SQL format interpolation, multiline chained command execution, raw pointer dereferencing in `unsafe` blocks, untrusted unwrap panics, and safe guard patterns.

---

## 🎯 Benchmark Purpose

1. **Rust-Specific Traps & Safety:** Catches dangerous `unsafe` blocks with unaligned/dangling pointer dereferencing, and unchecked `.unwrap()` calls on untrusted user inputs that cause thread panics.
2. **Multiline Chaining Analysis:** Tests review engine capability to parse multiline fluent builder APIs (such as `Command::new("sh").arg("-c").arg(cmd)`).
3. **Format-String SQL Injection:** Identifies `format!("SELECT ... WHERE id = {}", input)` queries.
4. **Zero False Positives:** Validates that safe parameterized execution (`Command::arg` arrays), proper `Result` pattern matching, and path traversal sanitizers produce **0 false positives**.

---

## 📋 Test Case Matrix

### 🔴 Security Vulnerabilities

| File | Issue / Vulnerability | Type | CWE | Severity | Expected |
| :--- | :--- | :--- | :--- | :---: | :---: |
| `query_builder.rs` | SQL Injection via raw `format!("SELECT ...")` interpolation | Injection | CWE-89 | High | **BLOCKING** |
| `cmd_exec.rs` | Command Injection via multiline `Command::new("sh").arg("-c")` | RCE | CWE-78 | High | **BLOCKING** |
| `file_storage.rs` | Path Traversal via unvalidated `Path::new(dir).join(filename)` | File Security | CWE-22 | High | **BLOCKING** |
| `memory_ops.rs` | Unsafe Raw Pointer Dereference (`*const i32`) & memory corruption | Memory Safety | CWE-476 / CWE-119 | High | **BLOCKING** |
| `auth_handler.rs` | Hardcoded JWT Secret Key & Plaintext Credential Logging (`eprintln!`) | Credential Exposure | CWE-798 / CWE-532 | High | **BLOCKING** |
| `http_fetcher.rs` | Server-Side Request Forgery (SSRF) via unvalidated URL | Network Security | CWE-918 | Medium | **BLOCKING** |
| `redirect_service.rs` | Open Redirect without host whitelist validation | Redirection | CWE-601 | Medium | **BLOCKING** |
| `cors_layer.rs` | Wildcard \`Any\` origin with \`allow_credentials(true)\` | CORS Misconfiguration | CWE-942 | High | **BLOCKING** |
| `xml_handler.rs` | XML reader without entity expansion controls (XXE) | Injection / XXE | CWE-611 | High | **BLOCKING** |
| `cookie_setter.rs` | Cookies explicitly built with \`http_only(false)\` and \`secure(false)\` | Insecure Cookie | CWE-614 / CWE-1004 | Medium | **NON-BLOCKING** |

### ⚡ Performance & Reliability Traps

| File | Issue | Type | Severity | Expected |
| :--- | :--- | :--- | :---: | :---: |
| `parser.rs` | Untrusted Input Panic via Unchecked `.unwrap()` | Runtime Denial of Service | Medium | **NON-BLOCKING** |
| `validator.rs` | Catastrophic Backtracking Regular Expression (ReDoS) | Algorithmic Complexity | Medium | **NON-BLOCKING** |

---

## 🛡️ Anti-False-Positive Guard File

| File | Safe Pattern Implemented | Expected Reviewer Result |
| :--- | :--- | :---: |
| `safe_guards.rs` | Parameterized `Command::args` (no shell invocation), path traversal guard checking `..`, explicit `Result`/`match` error handling, safe XML handling, hardened `http_only(true)` and `secure(true)` cookies | **0 False Positives** (Clean) |

---

## 🚀 How to Run the Benchmark

```bash
# View PR on GitHub
gh pr view 1 --web

# Trigger Review via API
curl -X POST http://localhost:8081/api/v1/review/trigger \
  -H "Content-Type: application/json" \
  -d '{
    "repository": "IlucielI/code-review-rust-test",
    "pull_request_id": 1
  }'
```

---

## 📊 Benchmark Validation Results

- **Detection Rate:** 14 / 14 (100%)
- **False Positive Rate:** 0 / 1 (`safe_guards.rs` completely passed)
- **False Negative Rate:** 0%
