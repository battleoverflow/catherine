<div align="center">
    <img src="https://raw.githubusercontent.com/azazelm3dj3d/catherine/main/assets/catherine_icon.png" width="40%" />
</div>

<h1 align="center">
    Catherine
</h1>

The Catherine Framework is a general-purpose cybersecurity framework built to provide extended support for defense operations. Catherine currently collects information from dumping hexadecimal content from files, validates malicious domains & IP addresses, attempts to crack unknown hashes, handles real-time database analysis, various types of decoding, and much more in a quick CLI utility. Thanks to Catherine being built in an easily packaged executable, you can quickly download the tool by running `cargo install catherine` via the `Cargo` ecosystem. Catherine can also be quickly compiled by pulling down the source code from `git` and simply running `cargo build` or `make` to build alongside modules.

NOTE: Modules do require Go, Python, and C build utilities to be installed.

Catherine provides a Command Line Interface (CLI) and Graphical User Interface (GUI) built into the executable. This means whether you install from source or `Cargo`, you can choose your method of use.

This project will most likely be in an experimental state for a long time. This project is a personal endeavor to explore different ways to control or manipulate data utilizing Rust. A more stable and production-oriented crate is available here: [Mercy](https://github.com/azazelm3dj3d/mercy). Mercy is an open-source Rust crate and CLI designed for building cybersecurity utilities and projects.

## Installation

You can easily install via the `Cargo` CLI:

```bash
cargo install catherine
```

If you'd prefer to install from source, you can also do this fairly easily, but it will still require the `Cargo` CLI.

First, clone the repository using `git`:

```bash
git clone https://github.com/azazelm3dj3d/catherine.git
```

Once you've cloned the repository and you're in the correct directory, simply run the following command:

```bash
cargo build
```

Now you'll have a local debug build available for testing under `target/debug/catherine`.

If you're interested in working with the Catherine modules, you can use the `make` build ecosystem to create executables for Catherine:

```bash
make
```

I am working on converting all external [modules](https://github.com/azazelm3dj3d/catherine-modules) (Python, C, Go) into native modules (Rust) to offer everything in a built-in executable via `Cargo` without any extra steps, but for now, I've made sure to keep them accessible (excluding the GUI) for extended functionality.

## Usage

If you've already installed the application from `Cargo`, all you have to do now is run the following command to initialize the Catherine shell:

```bash
catherine
```

If a GUI is more your style, there is a simple version available with the majority of the available CLI commands. All you have to do to launch the interface is initialize the Catherine shell and run the following command within the shell:

```bash
🦀 Catherine [v0.x.x] (None) ☀️  〉launch
```

If a bug or issue is found, please report it [here](https://github.com/azazelm3dj3d/catherine/issues).
