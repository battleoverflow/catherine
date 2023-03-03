<div align="center">
    <img src="https://raw.githubusercontent.com/azazelm3dj3d/catherine/main/assets/catherine_icon.png" width="40%" />
</div>

<h1 align="center">
    Catherine
</h1>

<!-- 📚 [Documentation](https://catherine.azazelm3dj3d.com) -->

Catherine is a blue team security framework with an extensible module collection, various types of cryptographic processes, hexadecimal dumping and aggregation, malicious domain detection, and real-time database analysis. Catherine is as simple as installing via Cargo or using `git` to pull down the source code and immediately start using the framework with `cargo run`. The modules for Catherine can be installed by using the `catherine_install` script or running the `install` command within the Catherine prompt. This requires sudo privileges.

One thing to note before installing, while Catherine should work on most operating systems, the modules are only built for Linux-based distributions.

### Installation
Install via Cargo (without modules):
```bash
cargo install catherine
```

Install via Cargo (with modules):
```bash
curl https://raw.githubusercontent.com/azazelm3dj3d/catherine/main/catherine_install > catherine_install && chmod +x catherine_install
```
```bash
./catherine_install
```

### Uninstall
If you need to uninstall Catherine on your system for any reason or would like to remove the external modules, you can do so with these commands:

Remove Catherine:
```bash
cargo uninstall catherine
```

Remove all Catherine files:
```bash
rm -r /opt/catherine
```

If a bug or issue is found, please report it [here](https://github.com/azazelm3dj3d/catherine/issues).