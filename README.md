# RustLearning

A concise, hands-on collection of Rust exercises and example projects used while learning Rust.

## Overview

This workspace contains chapter-by-chapter example projects (from beginner to intermediate) demonstrating core Rust concepts: project structure, data types, control flow, ownership & borrowing, functions, methods, and more.

Each subfolder under `chp*` contains one or more small Cargo projects you can build and run with `cargo`.

## Repository Structure (high level)

- `chp1/`, `chp2/`, `chp3/`, `chp4/`, `chp5/` — chapter folders with example projects.
- Each example is a Cargo workspace or binary crate with its own `Cargo.toml` and `src/main.rs`.

Example layout:

- `chp3/data_types/` — demonstrates Rust primitive types and formatting.
- `chp4/ownerships/` — ownership and move semantics examples.

## Prerequisites

- Rust toolchain (stable) installed via `rustup`.
- `cargo` available on your PATH.

Install Rust (if needed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## How to run

1. Open a terminal in the repo root.
2. Navigate to a chapter example, e.g.:

```bash
cd chp5/methods
cargo run
```

Or run directly from the workspace root by specifying the package path:

```bash
cd chp5/methods
cargo run --bin methods
```

Notes:
- Use `cargo run --release` to build optimized binaries.
- Remove `target/` directories if you need a clean rebuild.

## Contribution & Learning Notes

- These examples are intended for personal learning and experimentation. Feel free to copy, modify, and extend them.
- If you want improvements or additional examples (tests, exercises, or README clarifications), open an issue or send a patch.

## License

This repository is for learning purposes. Add a license if you intend to share or publish the code.

---

If you'd like, I can also add per-chapter READMEs, runnable examples, or short descriptions for each subproject — tell me which chapters to prioritize.
