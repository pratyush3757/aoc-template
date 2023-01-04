## Advent Of Code Template for Rust and Python
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)   

My template for [Advent of Code](https://adventofcode.com) challenges scaffolding in Rust and Python.  
Thanks to [AlexAegis's](https://github.com/AlexAegis/advent-of-code) and [Remlse's](https://github.com/remlse/cargo-templates) templates.  

Use with [cargo-generate](https://github.com/cargo-generate/cargo-generate).

### Usage
```sh
cargo generate aoc \
        --git "https://github.com/pratyush3757/aoc-template" \
        --branch main \
        --init
# or
cargo generate --init --path <local folder>
```

### Note:
Please copy the `./solutions/rust/lib` folder on first run.  
It has been ignored in the template, as cargo-generate cannot overwrite files and will throw errors on subsequent runs.

### TODO:
- [ ] Add python template code
