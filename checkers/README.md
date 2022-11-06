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
    - `git clone https://github.com/GIP2000/AI.git`

# Usage
1. `cd` into the `checkers` directory
2. `cargo run --release` will run the relase optimized version of the program.
3. `cargo run --features tree_debug` will output at the end of every move a json formatted tree
4. `cargo run --bin train --no-default-features --release` runs a training simulator to try and find better weights for heuristics

# Write up

## Board Implementation

For the game implementation I used a data structure that stores the board as a 2D 8x8 array of of enums which which represent a possible object that can be in the board. (Red King, Black King, Red, Black, and Empty.)
This datastructure also stores the another 2 instances of a PlayerInfo object which stores the player type (either RED or BLACK), a boolean to see if the player's moves are jump moves or not, a hashset of 2 integer tuples torepresenting the row and column of all that player's pieces, and a Vector of Moves Objects. The Moves Object stores a start and end location of the move as well as a hashset of pieces which will be jumped over if the move is a jump move. The board data structure also stores a refrence to which PlayerInfo is the current player.

The board datastructure has various methods responsible for implementing the gameplay. These methods include building a new board, refreshing the legal moves, and performing a move. A new Board can be initalized from either a string which is the readout of the file type or as a default board. The do move function takes in the index in the Vector representing which move should be done. After board creation, the current player is switched and that players moves are recalculated.

The legal moves are recalculated by looping through the HashSet of the current players pieces, then I check to see which move is legal, if the current move is a jump and we haven't seen a jump move yet, the vector is cleared and then I perform a DFS to determine the legal jumps for the player. I use HashSets in order to check for jumps for effeicently, by storing the pieces I jumped already I know if I already have jumped the piece.

The game loop is implemented in the bin/terminal_game.rs file Which runs the game loop and interacts with the player and optionally the Alpha/Beta Algorithms if selected.

## Alpha/Beta Algorithms & Heuristic

The Alpha/Beta program is implemented based off the pseudo code in the Slides.
I implemented a handful of heuristics.

1. A bonus for having material and a bonus if the piece is a king
2. A constant * how close a non-king piece is to the other side of the board.
    - I do not give this bonus after the piece is already a king.
3. A bonus for being in your own home row, a larger bonus for being in the center
4. A bonus for being in the center, with a larger one if your in the true center of the board.
5. A Mobility Bonus which gives a bonus for every move the player can do and a larger bonus if the moves are jump moves.
6. An Aggression multiplier, which uses the fraction of the ratio between the two players pieces ( larger amount of pieces / smaller amount of pieces) * a constant. This encourages trades if the player is ahead.
7. I also imlemented a penalty for the farthest disntace a from an opponents piece to any player's king. This encourages king aggresion
    - This hueristic was turned off for the final implmentation since it seemingly did not improve gameplay significantly and hurt performance.

## Training
I in an attempt to find better weight values I implmeented a mutation function for a hursitic object. I then spawned 50 threads and played the original huerstic against 50 different hurstics. The player in which wins the game first's huersitic heurstic moves one. This process is repeated for many geneations. For a while this was seemingly providing better results, however at the end my original values were always better than the mutated ones, and this was not used in the final implmentations, but the training program can still be run (See #4 in usage)

