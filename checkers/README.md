# Checkers Alpha Beta Pruning AI Projects
## Gregory Presser


1. Installation
2. Usage
3. Methodology


# Installation

1. Install Rust
    - Install Rustup (Linux / Macos)
        - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
        - run `cargo --version` to ensure rust is insalled properly
2. Download code.
    - `git clone git@github.com:GIP2000/AI.git`

# Usage
1. `cd` into the `checkers` directory
2. `cargo run --release` will run the relase optimized version of the program.
3. `cargo run --features tree_debug` will output at the end of every move a json formatted tree
4. `cargo run --bin train --no-default-features --release` runs a training simulator to try and find better weights for heuristics

# Methodology

