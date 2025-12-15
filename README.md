# Simple CLI mytodo list using rust
1. You can check this through command prompt with mytodo --help
2. Features: Add, List, Done, Remove

## How to use this through command prompt ?
1. type in terminal `cargo build --release`
2. Again with this:
### Linux/macOS
`./target/release/mytodo --help`
 
### Windows (PowerShell)
`.\target\release\mytodo.exe --help`

3. Make sure you already build release the mytodo application
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
4. Open new terminal and type `where mytodo`
5. Then you can running mytodo with `mytodo --help`

## Disclamer
- My folder name was mytodo, but you can change the name of it. 
- If you follow my instructions, don't forget to change every "mytodo" name to your folder name
