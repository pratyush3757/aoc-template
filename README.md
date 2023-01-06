## Advent Of Code Template for Rust and Python
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)   

My template for [Advent of Code](https://adventofcode.com) challenges scaffolding in Rust and Python.  
Thanks to [AlexAegis's](https://github.com/AlexAegis/advent-of-code) and [Remlse's](https://github.com/remlse/cargo-templates) templates.  

Use with [cargo-generate](https://github.com/cargo-generate/cargo-generate).

### Usage
```sh
cargo generate \
        --git "https://github.com/pratyush3757/aoc-template" \
        --branch master \
        --init
# or
cargo generate --init --path <local folder>
```

### Note:
Please copy the `./solutions/rust/lib` folder and `./Cargo.toml` on the first run.  
It has been ignored in the template, as cargo-generate cannot overwrite files and will throw errors on subsequent runs.

The `./Cargo.toml` file will act as the workspace toml file. Please edit the entries like `authors` accordingly.

### TODO:
- [ ] Add python template code
