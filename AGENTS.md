# AGENTS.md - RustyNotes Development Guide

This document provides guidance for agents working on the RustyNotes project.

## Project Overview

RustyNotes is a Rust and gtk-rs markdown note-taking application. The project is in its initial setup phase.

## Build Commands

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run

# Run with debug output
RUST_BACKTRACE=1 cargo run
```

## Test Commands

```bash
# Run all tests
cargo test

# Run a specific test by name
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc

# Run tests in release mode
cargo test --release
```

## Linting and Formatting

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Run Clippy lints
cargo clippy

# Run Clippy with warnings as errors
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

## Code Style Guidelines

### General Rust Conventions

- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use meaningful, descriptive names
- Keep lines under 100 characters when possible
- Use Rust 2021 edition or later

### Imports

- Use absolute paths with `crate::` for internal modules
- Group imports: standard library, external crates, local modules
- Use `use` statements for bringing items into scope
- Prefer importing specific items rather than modules

```rust
use std::fs::File;
use std::io::Read;

use gtk::prelude::*;
use gtk::{Application, Window};
```

### Error Handling

- Use `Result<T, E>` for functions that can fail
- Use the `?` operator for propagating errors
- Create custom error types using `thiserror` or `anyhow` for complex error handling
- Provide meaningful error messages

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### Types and Generics

- Use explicit type annotations where it improves readability
- Use generics for reusable code but don't over-generalize
- Prefer `&str` over `&String` in function signatures
- Use `Cow<str>` for strings that may be borrowed or owned

### gtk-rs Specific Guidelines

#### UI Construction

- Use Builder from `.ui` files when possible for complex layouts
- Keep UI construction separate from business logic
- Use the `gtk::prelude::*` trait for common GTK operations

```rust
use gtk::prelude::*;

fn create_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("RustyNotes");
    window.set_default_size(800, 600);
    window
}
```

#### Event Handling

- Use `connect_*` methods for signal handlers
- Use closures for simple handlers
- Extract complex handlers to separate methods
- Remember that GTK callbacks take ownership or clone data as needed

```rust
button.connect_clicked(|button| {
    button.set_label("Clicked!");
});
```

#### Async Operations

- Use `gio::` async variants for I/O operations
- Use `tokio` or `async-std` for async runtime if needed
- Keep UI responsive by offloading heavy work to background threads

### Documentation

- Document public APIs with doc comments (`///`)
- Include examples in documentation where helpful
- Document complex algorithms or business logic

### Testing

- Write unit tests in the same module using `#[cfg(test)]`
- Use integration tests in `tests/` directory for broader testing
- Test error conditions, not just success cases

### Performance Considerations

- Use `Rc<RefCell<T>>` or `Arc<Mutex<T>>` for shared mutable state
- Prefer clone-on-write types like `Cow` where applicable
- Use lazy initialization for expensive computations
- Profile before optimizing

## Project Structure (Recommended)

```
src/
├── main.rs           # Entry point
├── lib.rs            # Library root
├── app.rs            # Application logic
├── ui/
│   ├── mod.rs
│   ├── window.rs     # Main window
│   └── components/  # UI components
├── models/
│   ├── mod.rs
│   └── note.rs      # Note data model
├── storage/
│   ├── mod.rs
│   └── file.rs      # File operations
└── utils/
    └── mod.rs
```

## Dependencies (Recommended)

- `gtk = "0.5"` - GTK bindings
- `serde` / `serde_json` - Serialization
- `thiserror` - Error handling
- `tracing` / `tracing-subscriber` - Logging
- `pulldown-cmark` - Markdown parsing

## Security Considerations

- Never log sensitive information
- Validate all user input
- Use safe file path handling (avoid path traversal)
- Follow Rust's memory safety principles (no unsafe code unless necessary)
