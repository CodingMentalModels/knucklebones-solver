# Knucklebones Solver

Command Line Interface (CLI) solver for the [Cult of the Lamb](https://store.steampowered.com/app/1313140/Cult_of_the_Lamb/) minigame Knucklebones.  

## Knucklebones Rules

* Each player has a 3x3 grid of squares where they can place dice.  
* The players take turns rolling a die and then placing it.
* If a player places two dice of the same value on the same column, then the value of that column is doubled.  If three dice, then it's tripled.
* If a player places a die of the same value as one (or more) in the opposing column, the the opponent's dice of that value are removed.
* The game ends when one player has placed a die in each of the 9 squares.
* The winner is the one with the highest total, taking doubles and triples into account.

## Usage

Clone the Repo and run `cargo build` to compile it. 

Run `./target/debug/knucklebones-solver --help` from the root to see all of the options.

* `./target/debug/knucklebones-solver solve` Specify a position (die roll, player 1 board, player 2 board) and get the evaluation and optionally the full tree:

```
`knucklebones-solver.exe solve "___
___
___"

"___
___
___"

5
```

## Methodology

Because the game tree for Knucklebones is too big to brute force, we compute N moves ahead (4 by default) and then use a heuristic to min-max to approximate optimal play:
* For each player, we calculate the number of moves remaining if no eliminations occur.  
* We get a "moves remaining bonus" by multiplying the moves remaining by 3.5, an average die roll (1 + 2 + 3 + 4 + 5 + 6)/6
* We compute a modified score for each player as their current score plus the moves remaining bonus.
* We use the difference between the two players' modified scores as the evaluation of the current position, where positive numbers denote Player 1 winning and negative numbers denote Player 2 winning.

The motivation for this heuristic is that each player wants to maximize their score and minimize their opponent's score, but it's also critical that we take into account that a player with fewer empty squares will likely get to populate all of them, whereas her opponent will likely only get to populate a few before the game ends.  

Sources of error in the heuristic:
* The choice of the number 3.5 doesn't take into account any doubling or tripling of dice.  Even by chance, this will happen, and since players get to choose their best moves, it'll happen more than by chance.  This causes the heuristic to **understate** the expected point totals.
* The heuristic doesn't take elimination into account, which means that Position (the fact that the player who's going to finish first will have at least one more die played) is overvalued, since it's assumed to be permanent.  This causes the heuristic to **overstate** the expected point totals.  


## Insights

* Interesting situation: where should we put our 6?  On the one hand, (0, 2) keeps triple 6 outs alive; on the other, (1, 1) gives us 556 and 665 outs.  
```
./target/debug/knucklebones-solver.exe solve -d 5 -t 

"35_
___
___"

"__4
__4
___" 6
```
Answer: (1, 1) is better, because if opponent rolls a 6, we make him choose between eliminating ours and doubling his.  (0, 2) gives him an easy choice.

* The overvaluing of position seems to be a more important effect than the undervaluing of doubles and triples.  e.g. looking at a simple case below at depth 1 vs. depth 5, depth 5 is much closer to being even than depth 1.
```
./target/debug/knucklebones-solver.exe solve -d 5 -t 

"2__
___
___"

"___
___
___" 1
```

### Questions

