# CHIP-8 // Rust

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/github/license/A-str0/CHIP-8-Rust)
![Build](https://img.shields.io/github/actions/workflow/status/A-str0/CHIP-8-Rust/rust.yml?branch=dev)
![Issues](https://img.shields.io/github/issues/A-str0/CHIP-8-Rust)

A fully functional, high-performance, and clean CHIP-8 interpreter emulator written from scratch in modern Rust.

This project was created for educational purposes and as a personal challenge: to understand how virtual machines work, how low-level emulation operates, and to write idiomatic, safe, and fast code in Rust.

## 🚀 Quick Start

### Installation and Run

```bash
# Clone the repository and switch to dev branch
git clone https://github.com/A-str0/CHIP-8-Rust.git
cd CHIP-8-Rust

# Build and run (in release mode for performance)
cargo run --release

# Or in debug mode
cargo run
```

## 🎮 Controls

| PC Key | CHIP-8 Keypad |
|--------|---------------|
| 1 2 3 4| 1 2 3 C       |
| Q W E R| 4 5 6 D       |
| A S D F| 7 8 9 E       |
| Z X C V| A 0 B F       |

- `Esc/Del` — exit

## 📚 Where to Get ROMs?

Great collections:

- https://github.com/kripod/chip8-roms
- https://www.zophar.net/pdroms/chip8.html

## 📜 License

This project is licensed under the **GNU v2.0** License