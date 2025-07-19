# ff (Fast Finder)

A blazing-fast terminal utility to search for files or directories by name.

🚧 **Project is under active development**  
📚 **Built as a learning project to explore Rust programming and CLI design**

---

## Features

- 🔍 Search for files or directories by partial name match
- 📁 Option to include or limit to directories only
- 📂 Filter by file extension (e.g. `.rs`, `.txt`)
- 🧭 Control the search depth
- 🔠 Case-sensitive or case-insensitive matching (automatically adapts to OS)
- 💻 Simple and fast terminal interface using [clap](https://docs.rs/clap)

---

## Usage

```bash
ff search <filename> [OPTIONS]
```

## Examples

```bash
# Find any file or folder with 'config' in the name
ff search config

# Search only directories
ff search test --only-dirs

# Search files with a specific extension
ff search log -t txt

# Limit depth to 2 levels deep
ff search src -M 2

# Case-sensitive search (useful on Linux)
ff search README -i false

```
---

## Instalation

``` bash
git clone https://github.com/yourusername/ff.git
cd ff
cargo build --release
```

To have it in the terminal

https://doc.rust-lang.org/cargo/commands/cargo-install.html
Note: Later will add a bash script to install and uninstall it
---

## Why this project?

This tool was built to:

- Learn idiomatic Rust and project structuring
- Explore command-line argument parsing with clap
- Work with filesystem operations and recursion
- Understand unit/integration testing in Rust
- Build a small but useful real-world CLI tool

---

## Contributing

Suggestions, feedback, and improvements are welcome! Open an issue or submit a PR.

---

## License

MIT License


---

Let me know if you’d like to include badges (for crates.io, license, or CI), contribution guidelines, or more technical explanation of the internals (like how you use `walkdir` and how `max_depth` or `ignore_case` works).
