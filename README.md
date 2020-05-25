# Okay-Loc 
Its a binary that calculates how many lines of code you have written in a project.
Its git "aware" and like "okay".

### Git awerenss
Basically it excludes all files excluded in your .gitignore like `target` and such so they don't artificially inflate your numbers.
Additionally it excludes some none source code like files like `json` and `toml`.

### The comment counting is not really working, so thats WIP.

Run : `okay-loc --help` for up to date help.
OR you can try running `okay-loc -p ./` to scan your current directory.