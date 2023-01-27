<div align="center">
    <img src="https://raw.githubusercontent.com/CatherineFramework/Catherine/main/assets/catherine_icon.png" width="40%" />
</div>

<h1 align="center">
    Catherine
</h1>

📚 [Documentation](https://catherine.azazelm3dj3d.com)

Catherine is a blue team security framework with an extensible module collection, various types of cryptographic processes, hexadecimal dumping and aggregation, malicious domain detection, and real-time database analysis. Catherine is as simple as installing via Cargo or using `git` to pull down the source code and immediately start using the framework with `cargo run`. The modules for Catherine can be installed by using the `catherine_install` script or running the `install` command within the Catherine prompt. This requires sudo privileges.

### Installation
Install via Cargo (without modules):
```bash
cargo install catherine
```

Install via GitHub releases (with modules):
```bash
curl https://raw.githubusercontent.com/CatherineFramework/Catherine/catherine_install | sudo sh
```

## Final note
Catherine just entered a production (stable) state, so the documentation is currently being updated to match these new changes. For now, you can get started by installing via `Cargo` and viewing the help menu within the Catherine shell. If a bug or issue is found, please report it [here](https://github.com/CatherineFramework/Catherine/issues).