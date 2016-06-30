# Tic Tac Toe
A simple command line based Tic Tac Toe game written in rust.  
(This was just a programming exercise)

# Features
- two player Multiplayer
- single player against an AI
- unbeatable AI

# Example

```
$ ./target/release/tic-tac-toe
Tic Tac Toe
Should cross be controlled by the computer (y/n): no
Should circle be controlled by the computer (y/n): yes

*****************
   1   2   3
1    |   |   
  ---+---+---
2    |   |   
  ---+---+---
3    |   |   

*****************
round:  1
player: cross
move:   2 3

*****************
   1   2   3
1    |   |   
  ---+---+---
2    |   |   
  ---+---+---
3    | X |   

*****************
round:  2
player: circle  (computer player)
move:   2 1

*****************
   1   2   3
1    | O |   
  ---+---+---
2    |   |   
  ---+---+---
3    | X |   

*****************
round:  3
player: cross
move:   █
```

## Build

The software can easily be built with the rust package manager [cargo](https://crates.io):
```sh
cargo build --release
```
The executable can be found under `./target/release/tic-tac-toe`
