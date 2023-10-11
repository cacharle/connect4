# connect4

Connect 4 solver.

## Resources

* CodeBullet video - <https://www.youtube.com/watch?v=XRVA5PMSKKE>
* Pascal Pons blog - <http://blog.gamesolver.org/>

## TODO

- [ ] Better transposition table
- [ ] run in parallel
- [ ] sorting network for the best move sort
- [ ] find why I have more (3x) visited position than Mr. Pons
- [ ] Make a move predictor function that tries each possible moves and scores them
        Apparently you *have* to run it on each move which i don't understand
        (<https://github.com/PascalPons/connect4/issues/8>)
    But this would make parallelization very easy
- [ ] Generate an opening table (I think that's the only way of having a
  decently fast AI at the beginning of the game.
