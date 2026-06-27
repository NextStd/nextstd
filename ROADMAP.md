# NextStd Roadmap

**NextStd** is evolving from a memory-safe wrapper for `<stdio.h>` and
`<string.h>` into a comprehensive, modern systems toolkit for C developers.

The goal of this roadmap is to eliminate the need for complex POSIX APIs,
dangerous `system()` calls, and verbose external networking libraries by
offloading the heavy lifting to our safe Rust backend.

## Phase 0: Completed Core Features

The foundational architecture and core modules are fully implemented and ready
for production use.

* [x] **Type-Safe Printing (`ns_io`):** `ns_print` and `ns_println` functions
  with `_Generic` routing for `int`, `float`, `double`, and `char*`.
* [x] **Safe User Input:** `ns_read()` macro for dynamically capturing terminal
  input without buffer overflows.
* [x] **Memory-Safe Strings (`ns_string`):** Small String Optimization (SSO),
  dynamic scaling, safe concatenation, and length tracking.
* [x] **Crash-Proof Control Flow (`ns_error`):** Python-style `NS_TRY` and
  `NS_EXCEPT` macros, null-pointer protection, and `ns_error_t` enums.
* [x] **Safe Data Structures (`ns_data`):** Bounds-checked dynamic arrays
  (`ns_vec`) and Key-Value Maps (`ns_map`).
* [x] **Build & Architecture:** Cargo workspace integration, Makefile
  automation, and system-wide installation logic.

---

## Phase 1: Core Enhancements (Completed)

These features finalized the core `ns_io` and `ns_string` modules.

* [x] **String Interpolation & Formatting:** Introduce Python/Rust-style string
  formatting to eliminate the need for `sprintf`.
  * *Concept:* `ns_println("Value: {}", val);`

## Phase 2: Process Execution (`ns_cmd` & `ns_process`) (Completed)

Replacing standard C's `fork()`, `exec()`, and the highly insecure `system()`
calls with safe, memory-managed alternatives.

* [x] **The Better `system()` (`ns_cmd`):** A high-level execution macro that
  prevents shell injection and captures output safely without POSIX pipes.
  * *Architecture:* Introduces an `ns_cmd_output` struct containing separated
    `ns_string stdout` and `ns_string stderr` fields. This allows developers to
    parse clean data from `stdout` while independently handling error logs from
    `stderr`.
* [x] **Advanced Process Management (`ns_process`):** A wrapper around Rust's
  `std::process::Command` using a safe builder pattern.
  * *Features:* Similar to `ns_cmd` but allows for non-blocking concurrent
    orchestration, background execution, and process termination polling.

## Phase 3: Networking (`ns_http`) (Completed)

Bringing modern, memory-safe HTTPS requests to C without the massive boilerplate
of `libcurl`.

* [x] **Simple GET & POST Requests:** Wrapper around Rust's `reqwest` to easily
      fetch HTML or JSON APIs directly into dynamically sized, auto-cleaning
      `ns_string` structures.
* [ ] **Bot Protection Bypass:** Built-in modern browser User-Agent injection to
      seamlessly bypass basic Cloudflare and bot-protection filters.
* [x] **Safe Error Propagation:** Catches dropped connections, invalid UTF-8
      responses, and network failures safely via `NsError` instead of crashing
      the program. Ensures payload data integrity by safely extracting raw
      bytes.

## Phase 4: Safe File System (`ns_file`)

Fixing the resource leaks and buffer overflow risks associated with standard C's
`FILE*` API.

* [ ] **One-Shot File I/O:** High-level functions like `ns_file_read_to_string`
  that automatically open a file, calculate its size, dynamically allocate
  an `ns_string`, and safely close the handle.
* [ ] **Safe Streaming:** Opaque `ns_file_t` structs managed entirely by the
  Rust backend, allowing C to safely read large files in chunks without ever
  touching raw pointers or leaking descriptors.

## Phase 5: Terminal UI (`ns_tui`)

Bringing modern, declarative terminal interfaces to C, bypassing the complexity
and unsafety of `ncurses`.

* [ ] **Declarative TUI API:** Wrapping Rust's `ratatui` crate using an
      immediate-mode Command Buffer API in C.
* [ ] **Modern Layouts:** Render blocks, paragraphs, and flex layouts natively
      in the terminal.

## Phase 6: C++ Integration

* [ ] **C++ Support:** Ensure seamless compatibility for using NextStd macros
  and functions directly in C++ codebases.

---

## The Ultimate Stress Test: CLI Media Scraper

As these modules come online, they will be stress-tested by building a complex
interactive CLI application.

**Concept:** A terminal-based scraper for media sites.

1. Use `ns_io` to capture search queries safely.
2. Use `ns_http` to fetch and parse the streaming API data safely, bypassing bot
   protections.
3. Use `ns_tui` to build a clean, interactive dashboard to navigate search
   results.
4. Use `ns_cmd` / `ns_process` to concurrently orchestrate an external player
   (like `mpv`) for direct playback without blocking the C main thread.
