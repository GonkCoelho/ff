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
