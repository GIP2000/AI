# Checkers Alpha Beta Pruning AI Projects
## Gregory Presser


1. Installation
2. Usage
3. Methodology


# Installation

1. Install Rust
    - Install Rustup (Linux / Macos)
        - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
        - run `cargo --version` to ensure rust is installed properly
2. Download code.
    - `git clone https://github.com/GIP2000/AI.git`

# Usage
1. `cd` into the `checkers` directory
2. `cargo run --release` will run the release optimized version of the program.
3. `cargo run --features tree_debug` will output at the end of every move a json formatted tree
4. `cargo run --bin train --no-default-features --release` runs a training simulator to try and find better weights for heuristics

# Write up

## Board Implementation

For the game implementation I used a data structure that stores the board as a 2D 8x8 array of enums which represent a possible object that can be in the board. (Red King, Black King, Red, Black, and Empty.)
This data structure also stores the another 2 instances of a PlayerInfo object which stores the player type (either RED or BLACK), a boolean to see if the player's moves are jump moves or not, a hashset of 2 integer tuples representing the row and column of all that player's pieces, and a Vector of Moves Objects. The Moves Object stores a start and end location of the move as well as a hashset of pieces which will be jumped over if the move is a jump move. The board data structure also stores a reference to which PlayerInfo is the current player.

The board data structure has various methods responsible for implementing the gameplay. These methods include building a new board, refreshing the legal moves, and performing a move. A new Board can be initialized from either a string which is the readout of the file type or as a default board. The do move function takes in the index in the Vector representing which move should be done. After board creation, the current player is switched and that players moves are recalculated.

The legal moves are recalculated by looping through the HashSet of the current players pieces, then I check to see which move is legal, if the current move is a jump and we haven't seen a jump move yet, the vector is cleared and then I perform a DFS to determine the legal jumps for the player. I use HashSets in order to check for jumps efficiently, by storing the pieces I jumped already. This allows for both efficient removal of jumped pieces when the move is selected while also allowing me to know in constant time if I already jumped over a given piece. 

The game loop is implemented in the bin/terminal_game.rs file Which runs the game loop and interacts with the player and optionally the Alpha/Beta Algorithms if selected.

## Alpha/Beta Algorithms

The Alpha/Beta program is implemented based off the pseudo code in the Slides. In addition to the normal min max algorithm with alpha beta pruning I also implemented a tree debugger. This feature stores the state of the game tree and outputs it to a json file. This feature significantly slows down the performance of the application and is therefore under macros which conditionally compile it into the binary. When running it in tree_debug mode (Usage #3) a json representation of all the relevant information will be outputted after every move. I also made a react application to allow me to view the json format of the tree visually. This was useful in the beginning when debugging but is now useless since the tree is too big to be stored in a browser's memory. 

## Heuristics

I implemented a handful of heuristics to improve the performance of the program. The scores for a board land in the 32 bit integer range, where a min is equal to the max 32 bit integer and a loss is equal to its negative complement. I also give a penalty for a win that is equal to how far in the depth the win is, encouraging a faster win, and the opposite for a loss (encouraging a slower loss). I also added a random value to the end of each heuristic between -9 and 9. Since all my weights are far larger, this should not change the difference between different values unless they were evaluated to be the same number ensuring the same move is not picked multiple times. 

1. A bonus for having material and a bonus if the piece is a king
2. A constant * how close a non-king piece is to the other side of the board.
    - I do not give this bonus after the piece is already a king.
3. A bonus for being in your own home row, a larger bonus for being in the center
4. A bonus for being in the center, with a larger one if your in the true center of the board.
5. A Mobility Bonus which gives a bonus for every move the player can do and a larger bonus if the moves are jump moves.
6. An Aggression multiplier, which uses the fraction of the ratio between the two players pieces ( larger amount of pieces / smaller amount of pieces) * a constant. This encourages trades if the player is ahead.
7. I also implemented a penalty for the farthest distance from an opponent's piece to any player's king. This encourages king aggression
    - This heuristic was turned off for the final implementation since it seemingly did not improve gameplay significantly and hurt performance.

## Training
I in an attempt to find better weight values I implemented a mutation function for a heuristic object. I then spawned 50 threads and played the original heuristic against 50 different heuristics. The player who wins the game's first heuristic moves on. This process is repeated for many generations. For a while this was seemingly providing better results, however at the end my original values were always better than the mutated ones, and this was not used in the final implementations, but the training program can still be run (See #4 in usage)

## Issues
I have noticed that when given a board where a king is given to the player too early and I have the computer play against itself the two agents reach a point where either player must give up material, or they can both circularly move their kings back and forth, creating an infinite loop. This issue can likely be solved with better heuristics.
