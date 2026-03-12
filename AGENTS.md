# AGENTS.md - RustyNotes Development Guide

## Project Overview
RustyNotes is a Rust-based desktop note-taking application using the Iced GUI framework with markdown support. This is a new project - conventions follow standard Rust best practices.

## Build, Lint, and Test Commands

### Basic Commands
```bash
cargo build                    # Build the project
cargo build --release         # Build in release mode
cargo run                     # Run the application
cargo run --release           # Run in release mode
```

### Testing
```bash
cargo test                    # Run all tests
cargo test test_name          # Run a single test by name
cargo test -- --nocapture     # Run tests with output
cargo test -v                 # Run tests with verbose output
cargo test --doc              # Run doc tests
```

### Linting and Code Quality
```bash
cargo clippy                  # Run Clippy lints
cargo clippy -- -D warnings   # Run Clippy with warnings as errors
cargo clippy --release        # Clippy for release target
cargo check                   # Check code compiles (faster than build)
cargo check --release         # Check in release mode
```

### Formatting
```bash
cargo fmt                     # Format all code
cargo fmt -- --check          # Check formatting without making changes
```

### Combined Commands
```bash
cargo fmt -- --check && cargo clippy && cargo test  # Full check before committing
```

### Documentation
```bash
cargo doc                     # Generate documentation
cargo doc --open              # Generate and open documentation
```

## Code Style Guidelines

### General Principles
- Use Rust 2021 edition (change from 2024 in Cargo.toml if needed)
- Enable all lints: `#![warn(clippy::all, clippy::pedantic)]`
- Prefer explicit over implicit
- Write self-documenting code with clear variable/function names

### Formatting
- Use 4 spaces for indentation, max line length: 100 characters
- Use Rustfmt: `cargo fmt`

### Imports
```rust
// Group by crate with blank lines between categories
use std::collections::HashMap;
use std::fmt;

use iced::Element;
use serde::{Deserialize, Serialize};

use crate::module::function;
```

### Naming Conventions
- Variables/functions: snake_case (`note_content`, `save_note`)
- Types/Structs/Enums: PascalCase (`Note`, `NoteState`)
- Constants: SCREAMING_SNAKE_CASE (`MAX_NOTE_LENGTH`)
- Files/Modules: snake_case (`note_manager.rs`, `mod note_manager`)
- Traits: adjectives or nouns (`Clone`, `Renderable`)

### Types
- Use explicit type annotations for public APIs
- Prefer `&str` over `&String` for parameters, return `String` for owned data
- Use `Arc<T>` for shared ownership, `Rc<T>` for single-threaded

### Error Handling
```rust
fn read_note(path: &Path) -> Result<Note, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(Note { content })
}

// Use thiserror for custom error types
#[derive(thiserror::Error, Debug)]
pub enum NoteError {
    #[error("Failed to read note: {0}")]
    ReadError(#[from] io::Error),
    #[error("Note not found: {0}")]
    NotFound(String),
}

// Avoid unwrap/expect in production; use ? or unwrap_or_default()
```

### Testing
- Write tests in `#[cfg(test)]` modules in the same file
- Use descriptive names: `#[test] fn test_save_note_truncates_long_content()`

### Documentation
- Document public APIs with doc comments: `/// Loads a note from disk`
- Include examples for important functions

### Iced-Specific Guidelines
- Use `Element<'_, Message>` for UI components
- Implement `From<T>` for messages
- Keep state immutable, use `Command` for side effects
- Follow Iced's component model

### Dependencies
- Pin critical dependencies in Cargo.toml
- Use minimal dependencies; prefer std
- Add with: `cargo add <crate>`

### Git Conventions
- Use conventional commits: `feat: add note save functionality`
- Run `cargo fmt` and `cargo clippy` before committing

## Project Structure (Recommended)
```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library root
├── commands/         # CLI commands
├── models/           # Data structures
├── services/         # Business logic
├── ui/               # Iced widgets/components
│   ├── widgets/
│   └── screens/
└── utils/            # Helper functions
```

## Performance Considerations
- Use `cargo build --release` for production
- Use `#[inline]` for small, frequently called functions
