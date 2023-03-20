<div align="center">
    <img src="https://raw.githubusercontent.com/azazelm3dj3d/catherine/main/assets/catherine_icon.png" width="40%" />
</div>

<h1 align="center">
    Catherine
</h1>

<!-- 📚 [Documentation](https://catherine.azazelm3dj3d.com) -->

The Catherine Framework is a general-purpose cybersecurity framework built to aggregate, validate, decode, decrypt, and maintain data. Catherine currently collects information from hexadecimal data dumps, validates malicious domain/IP detection, attempts to crack unknown strings and hashes, handles real-time database analysis, and much more! Thanks to Catherine being built in an easily packaged executable, you can quickly download the tool by running `cargo install catherine` via the `cargo` ecosystem. Catherine can also be quickly built by pulling down the source code from `git` and simply running `cargo build`.

Catherine provides a Command Line Interface (CLI) and Graphical User Interface (GUI) built into the executable. This means whether you install from source or `Cargo`, you can choose your method of use.

## Installation

You can easily install via the `Cargo` CLI utility:
```bash
cargo install catherine
```

If you'd prefer to install from source, you can also do this fairly easily, but it will still require the `Cargo` utility.

First, clone the repository using `git`:

```bash
git clone https://github.com/azazelm3dj3d/catherine.git
```

Once you've cloned the repository and you're in the correct directory, simply run the following command:

```bash
cargo build
```

Now you'll have a local debug build available for testing.

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