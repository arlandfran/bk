# [b]ash[k]eys

A simple CLI that provides a quick reference to ash keyboard shortcuts, written in Rust.

## Features

- 📚 Comprehensive collection of bash keyboard shortcuts
- 🗂️ Organized by categories (movement, edit, history, process)
- 🚀 Fast and lightweight
- 🎯 Simple and intuitive interface
- ✅ Well-tested and documented code

## Installation

### Prerequisites
- Rust 1.70+ installed on your system
- Cargo (comes with Rust)

### Build from source

```bash
# Clone the repository
git clone https://github.com/yourusername/bk.git
cd bk

# Build the project
cargo build --release

# The binary will be available at target/release/bk
# Optionally, install it to your PATH
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Show all shortcuts
bk

# Show movement related shortcuts
bk -m
bk --movement

# Show edit related shortcuts  
bk -e
bk --edit

# Show command recall (history) related shortcuts
bk -r
bk --recall

# Show process related shortcuts
bk -p
bk --process

# Show version information
bk -v
bk --version

# Show help
bk -h
bk --help
```

### Combine Categories

You can combine multiple category flags in two ways:

**Separate flags:**
```bash
# Show both movement and edit shortcuts
bk -m -e

# Show recall and process shortcuts
bk --recall --process
```

**Chained flags (Unix-style):**
```bash
# Show movement and edit shortcuts (chained)
bk -me

# Show movement, edit, and recall shortcuts
bk -mer
```

## Shortcut Categories

### Movement Shortcuts
- Cursor navigation and positioning
- Jump between words and lines
- Toggle cursor positions

### Edit Shortcuts  
- Text editing and manipulation
- Cut, copy, paste operations
- Case transformations
- Undo/redo functionality

### Command Recall (History) Shortcuts
- Command history navigation
- History search and recall
- Command repetition and substitution

### Process Shortcuts
- Process control and management
- Signal handling (SIGINT, SIGTSTP)
- Screen control operations

## Examples

```bash
# Quick reference for all shortcuts
$ bk

# Just show me movement shortcuts
$ bk -m
=== MOVEMENT Shortcuts ===
  Ctrl+a       Go to the beginning of the line (Home)
  Ctrl+e       Go to the End of the line (End)
  Ctrl+f       Forward one character (Right arrow)
  ...

# Show movement and edit shortcuts (chained flags)
$ bk -me
=== MOVEMENT Shortcuts ===
  Ctrl+a       Go to the beginning of the line (Home)
  ...
=== EDIT Shortcuts ===
  Ctrl+L       Clear the Screen, similar to the clear command
  ...

# Show edit and recall shortcuts together
$ bk -e -r
=== EDIT Shortcuts ===
  Ctrl+L       Clear the Screen, similar to the clear command
  Alt+Del      Delete the Word before the cursor
  ...
=== RECALL Shortcuts ===  
  Ctrl+r       Recall the last command including the specified character(s)
  ...
```

## Development

### Running Tests

```bash
# Run unit tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

### Code Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Project configuration and dependencies
- Tests are included inline with the `#[cfg(test)]` attribute

### Design Principles

This CLI follows the [Command Line Interface Guidelines](https://clig.dev/) for good CLI design:

- ✅ **Be consistent** - Similar patterns for all flags
- ✅ **Be helpful** - Clear help text and examples
- ✅ **Be fast** - Minimal startup time and dependencies
- ✅ **Be robust** - Proper error handling and validation
- ✅ **Be empathetic** - Intuitive interface and good defaults

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References

- [SS64 Bash Keyboard Shortcuts](https://ss64.com/mac/syntax-bashkeyboard.html) - Source of the shortcuts
- [Command Line Interface Guidelines](https://clig.dev/) - CLI design principles
- [Clap Documentation](https://docs.rs/clap/latest/clap/) - Argument parsing library
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Rust language reference
