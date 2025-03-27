---
title: Build from Source Code
description: A guide to build mojiji from source code.
---

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager
- Git
- Basic command-line knowledge

## Build Steps

### 1. Clone the Repository

```bash
git clone https://github.com/HidemaruOwO/mojiji.git
cd mojiji
```

### 2. Compile the Project

```bash
cargo build --release
```

This command compiles the project with optimizations for production use.

### 3. Install the Binary

```bash
# Install to system-wide location
sudo install -Dm0755 -t "/usr/local/bin/" "target/release/mojiji"
```

The `install` command copies the compiled binary to a system-wide executable location.

## Running Mojiji

### Start Local Server

```bash
# Runs Mojiji on localhost port 8000
mojiji
```

### Additional Notes

- The server will be accessible at `http://localhost:8000`
- Ensure you have the necessary permissions to install system-wide binaries
- No additional configuration is required for basic setup
