# 🦀 Mytodo - Simple CLI Task Manager

A simple command-line interface (CLI) application to manage your todo list, built with **Rust**.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)

## ✨ Features
* **Add**: Create new tasks easily.
* **List**: View all your pending and completed tasks.
* **Done**: Mark tasks as completed.
* **Remove**: Delete tasks permanently.
* **Persistent**: Data is saved automatically to `db.json`.

---

## 🚀 Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed.

### Build form Source
1.  Clone this repository.
2.  Build the release version:
    ```bash
    cargo build --release
    ```
3.  The binary will be available in `./target/release/mytodo`.

### Linux/macOS
`./target/release/mytodo --help`
 
### Windows (PowerShell)
`.\target\release\mytodo.exe --help`

---

## 📦 Installation (Add to PATH)

To run `mytodo` from anywhere in your terminal, follow these steps according to your OS.

### Linux
```
mkdir -p ~/.local/bin
cp target/release/mytodo ~/.local/bin/
chmod +x ~/.local/bin/mytodo
```
#### make sure PATH loads ~/.local/bin (bash)
```
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
which mytodo
```
### macOS
```
mkdir -p ~/bin
cp target/release/mytodo ~/bin/
chmod +x ~/bin/mytodo
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc
which mytodo
```
### Windows
```
New-Item -ItemType Directory -Path "$env:USERPROFILE\bin" -Force | Out-Null
Copy-Item target\release\mytodo.exe "$env:USERPROFILE\bin\"
$p = [Environment]::GetEnvironmentVariable("Path","User")
if ($p -notlike "*$env:USERPROFILE\bin*") {
    [Environment]::SetEnvironmentVariable("Path","$env:USERPROFILE\bin;$p","User")
}
```
## Open new terminal and type 
`where mytodo`
## Then you can running mytodo with 
`mytodo --help`

---

# ⚠️ Disclamer
- My folder name was mytodo, but you can change the name of it. 
- If you follow my instructions, don't forget to change every "mytodo" name to your folder name
