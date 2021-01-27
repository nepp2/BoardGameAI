
# Rust workshop demo

This is a small demo application featuring:

* A generic turn-based game interface
* A generic interface for AI agents to play these games
* A graphical implementation of Checkers and TicTacToe
* An agent that takes random actions
* A simplistic rollout-based agent that can play these games

## Install Rust

https://www.rust-lang.org/tools/install

**Optional**: Install an editor with Rust support, like VS Code and its Rust extension. Alternatively just use any simple code editor.

## Download the workshop

e.g. open a terminal and run `git clone https://github.com/nepp2/rust_workshop`

## Build the project

Navigate to the project folder (which contains the `Cargo.toml` file).

Type `cargo build`.

(This is a debug build. For an optimised build, try `cargo build --release`)

## Run something

Try one of the following commands:

* `cargo run checkers`
  * Opens graphical checkers game
  * Play manually with the mouse
  * Press Space to trigger an AI move
* `cargo run tictactoe`
  * Opens graphical tictactoe
  * Similar controls to checkers
* `cargo run contest`
  * Two agents play 100 games of checkers against each other

(Again, for an optimised build, try `cargo run --release contest`)

## Troubleshooting

Try `rustup update` to make sure you have the latest version of Rust.

If it still doesn't work, just heckle me for preparing a shoddy demo project, I guess.





