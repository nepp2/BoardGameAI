
# Rust workshop demo

This is a small demo application featuring:

* A generic turn-based game interface
* A generic interface for AI agents to play these games
* A graphical implementation of Checkers and TicTacToe
* An agent that takes random actions
* A simplistic rollout-based agent that can play these games

## Install Rust

https://www.rust-lang.org/tools/install

**Optional**: Install an editor with Rust support, like VS Code and its `Rust (rls)` extension. Alternatively just use any simple code editor.

## Download the workshop

e.g. open a terminal and run `git clone https://github.com/nepp2/rust_workshop`

## Build the project

Navigate to the project folder (which contains the `Cargo.toml` file).

Type `cargo build --release`.

(This is an optimised build. For a debug build, try `cargo build`)

## Run something

Try one of the following commands:

* `cargo run --release checkers`
  * Opens graphical checkers game
  * Play manually with the mouse
  * Press Space to trigger an AI move
* `cargo run --release tictactoe`
  * Opens graphical tictactoe
  * Similar controls to checkers
* `cargo run --release contest`
  * Two agents play 100 games of checkers against each other

## Troubleshooting

Try `rustup update` to make sure you have the latest version of Rust.

If it still doesn't work, just heckle me for preparing a shoddy demo project, I guess.





