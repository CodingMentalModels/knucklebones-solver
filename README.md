# Knucklebones Solver

## Rules

* Each player has a 3x3 grid of squares where they can place dice.  
* The players take turns rolling a die and then placing it.
* If a player places two dice of the same value on the same column, then the value of that column is doubled.  If three dice, then it's tripled.
* If a player places a die of the same value as one (or more) in the opposing column, the the opponent's dice of that value are removed.
* The game ends when one player has placed a die in each of the 9 squares.
* The winner is the one with the highest total, taking doubles and triples into account.

## Usage

* Specify a position (die roll, player 1 board, player 2 board) and get the optimal placement:

`knucklebones-solver.exe solve 5 "___
___
___"

"___
___
___"

## Design

### Implement the primitives
* Players
* Squares
* Dice
* Score

### Build the game tree

* Build the tree in a way that prunes bad branches and reduces the scale of overall tree to something many orders of magnitude less than 6^18*(9!)^2.  
* Alpha-beta search?  

### Solve the game

### Expose a Command Line Interface to allow a user to solve the game from any position

