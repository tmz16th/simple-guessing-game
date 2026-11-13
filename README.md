# rust

My Rust learning projects, following [The Rust Programming Language](https://doc.rust-lang.org/book/).

## Projects

### [hello_world](./hello_world)
The first program, written two ways: a single `main.rs` compiled directly with `rustc`, and the same thing as a Cargo project in `hello_cargo/`.

### [guessing_game](./guessing_game)
Chapter 2's number guessing game. It picks a random number from 1 to 100 and tells you whether each guess is too high or too low.

### [variables](./variables)
Small experiments with `let`, `mut`, and changing a variable's value.

### [hangman](./hangman)
A terminal Hangman game. Guess one letter at a time, or type the whole word if you know it. The ASCII gallows art lives in its own module, `src/gallows.rs`.

## Running a project

Every folder except `hello_world/main.rs` is a Cargo project:

```sh
cd guessing_game
cargo run
```
