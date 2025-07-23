# bashkey (bk)

![bk_demo](https://github.com/user-attachments/assets/c5a9f26e-c3a4-49a7-a7cd-bff44fc85236)

## Installation

### Prerequisites
- Rust 1.70+ installed on your system
- Cargo (included in Rust toolchain)

### 

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

```bash
# Quick reference for all shortcuts
$ bk

# Just show me movement related shortcuts
$ bk -m

# Show movement and edit related shortcuts (chained flags)
$ bk -me

# Filter ctrl key shortcuts
$ bk | grep ctrl

# Uninstall bk
$ bk --uninstall
```

## References

- [SS64 Bash Keyboard Shortcuts](https://ss64.com/mac/syntax-bashkeyboard.html) - Shortcuts source
- [Command Line Interface Guidelines](https://clig.dev/) - CLI design principles
- [Clap Documentation](https://docs.rs/clap/latest/clap/) - Argument parsing library
- [Tabled Documentation](https://github.com/zhiburt/tabled/) - Pretty print table library
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Rust language reference

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
