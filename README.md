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
* Notice that the game is symmetric along columns -- i.e. you can pick any row to put a given die in in a given column and the resulting position is equivalent.
* Three possible approaches:
    * Based on the current decision point, eliminate dominated strategies and then continue constructing the tree.  Only works if:
        * There's a reliable way to identify dominated strategies
        * Removing dominated strategies results in a small enough tree -- This failes to be true since 6^17 is huge.  
    * Build the subtree n moves out and then heuristically evaluate the positions.  
        * Need to pick a depth
        * Need a heuristic -- Considerations:
            * What's the value of the die?
            * Does this die double or triple a column?
            * Does this die eliminate opponents' dice and if so, how many?
            * Is the game about to end?
    * Regret Minimization -- Run N iterations of the game and after each one update the player's strategies to use the one that performed the least badly against opponent's maximally exploitive strategy.    
* Let's go with the heuristic-based approach, with the following heuristic:
    * If we can brute force the game, i.e. we're some small number of moves from the end (perhaps 3 moves per player, which makes the tree < 6^6*(3!)^2 nodes, which should be managable).
    * Heuristic = difference between the player's score and opponent's score, assuming that each empty space is populated with an average die (3.5)


Tricky Case:
How should we compare the following moves?
* Move 1: Half of opponent's rolls lead to forced win for us, half lead to an evaluation (heuristic) of 0.
* Move 2: All of opponent's rolls lead to evaluation of +N (we're winning by N).

Unclear what N should be to make us indifferent, but this should happen very rarely and when it does it will usually be extremely clear how we should compare the cases (e.g. eliminating 3 opponent's 5s vs. tripling our on 5s is clearly better).  So, let's give a bonus of 3 to forced winning line.

### Solve the game

### Expose a Command Line Interface to allow a user to solve the game from any position

