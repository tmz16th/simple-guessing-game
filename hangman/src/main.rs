// A simple Hangman game.
//
// New Rust ideas used here:
//   - `Vec<char>` : a growable list of letters we've already guessed
//   - `&str` vs `String` : borrowed text vs. owned, growable text
//   - `Option<char>` : "a letter, or nothing if the input was bad"
//   - `enum Guess` : input can be one of a few different kinds of value
//   - helper functions that take references (`&str`, `&[char]`)

use std::io::{self, Write};

use rand::prelude::*;

// Pull in the other file in this folder, `src/gallows.rs`.
mod gallows;

/// How many wrong guesses the player gets before losing.
const MAX_WRONG: usize = 6;

/// The pool of secret words to choose from.
const WORDS: [&str; 12] = [
    "rust",
    "crab",
    "borrow",
    "compiler",
    "cargo",
    "lifetime",
    "ownership",
    "trait",
    "enum",
    "struct",
    "closure",
    "iterator",
];

/// What the player typed: either a single letter, or a whole word.
///
/// An `enum` describes a value that is exactly one of a fixed set of
/// shapes. `match` lets us handle each shape separately.
enum Guess {
    Letter(char),
    Word(String),
}

fn main() {
    println!("=== Hangman ===");
    println!("Guess the secret word one letter at a time --");
    println!("or type the whole word if you think you know it.");
    println!("You may make {MAX_WRONG} wrong guesses.\n");

    // Play one round, then ask whether to play again.
    loop {
        play_round();

        if !play_again() {
            println!("Thanks for playing!");
            break;
        }
        println!();
    }
}

/// Play a single game, from picking the word to winning or running out of guesses.
fn play_round() {
    // `secret` is a &str borrowed from the WORDS constant, so it costs
    // nothing to copy around -- it just points at existing text.
    let secret: &str = pick_word();

    // Letters the player has already tried. Starts empty and grows.
    let mut guessed: Vec<char> = Vec::new();
    let mut wrong: usize = 0;

    loop {
        println!("{}", gallows::for_mistakes(wrong));
        println!();
        println!("Word: {}", masked_word(secret, &guessed));
        println!("Wrong guesses left: {}", MAX_WRONG - wrong);

        if !guessed.is_empty() {
            println!("Letters tried: {}", letters_to_string(&guessed));
        }
        println!();

        // Check for the end of the game before asking for more input.
        if is_solved(secret, &guessed) {
            println!("You win! The word was \"{secret}\".\n");
            return;
        }
        if wrong == MAX_WRONG {
            println!("Out of guesses. The word was \"{secret}\".\n");
            return;
        }

        let guess = match read_guess() {
            Some(g) => g,
            None => {
                println!("Type a single letter, or the whole word.\n");
                continue;
            }
        };

        match guess {
            Guess::Letter(letter) => {
                if guessed.contains(&letter) {
                    println!("You already tried '{letter}'.\n");
                    continue;
                }

                guessed.push(letter);

                if secret.contains(letter) {
                    println!("Good guess! '{letter}' is in the word.\n");
                } else {
                    wrong += 1;
                    println!("Sorry, no '{letter}'.\n");
                }
            }
            Guess::Word(word) => {
                if word == secret {
                    // Reveal every letter. The win check at the top of the
                    // loop will then print the board and the winning message.
                    for c in secret.chars() {
                        if !guessed.contains(&c) {
                            guessed.push(c);
                        }
                    }
                } else {
                    wrong += 1;
                    println!("Nope, \"{word}\" is not the word. That costs one guess.\n");
                }
            }
        }
    }
}

/// Ask whether to play another round. Returns false on "n" or end of input.
fn play_again() -> bool {
    print!("Play again? (y/n) ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // End of input (Ctrl+D): stop rather than loop forever.
    if bytes_read == 0 {
        return false;
    }

    let answer = input.trim().to_ascii_lowercase();
    answer == "y" || answer == "yes"
}

/// Pick one random word out of `WORDS`.
///
/// The return type `&'static str` means "a borrowed string that lives
/// for the whole program" -- exactly what a constant is.
fn pick_word() -> &'static str {
    let mut rng = rand::rng();
    let index = rng.random_range(0..WORDS.len());
    WORDS[index]
}

/// Build the word as the player sees it: known letters shown, the rest as `_`.
///
/// `secret` is borrowed with `&str` and `guessed` with `&[char]` (a slice),
/// so this function reads the data but never takes ownership of it.
fn masked_word(secret: &str, guessed: &[char]) -> String {
    let mut out = String::new();

    for (i, c) in secret.chars().enumerate() {
        if i > 0 {
            out.push(' '); // put a space between letters
        }
        if guessed.contains(&c) {
            out.push(c);
        } else {
            out.push('_');
        }
    }

    out
}

/// True once every letter of the secret word has been guessed.
fn is_solved(secret: &str, guessed: &[char]) -> bool {
    secret.chars().all(|c| guessed.contains(&c))
}

/// Turn the guessed letters into one string for display, e.g. "a r t".
fn letters_to_string(letters: &[char]) -> String {
    let mut out = String::new();
    for (i, c) in letters.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push(*c); // `c` is a &char here, so `*c` gives us the char itself
    }
    out
}

/// Ask the player for one letter or a whole word.
///
/// Returns `Some(Guess)` for good input, or `None` if the input was empty
/// or contained anything that is not a letter.
fn read_guess() -> Option<Guess> {
    // `print!` (without ln) does not add a newline, so we flush the
    // buffer to make sure the prompt actually appears before we wait.
    print!("Your guess: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // `read_line` returns 0 bytes when the input is finished (the player
    // pressed Ctrl+D, or a script ran out of lines). Without this check the
    // loop below would spin forever, so just stop.
    if bytes_read == 0 {
        println!("\nGoodbye!");
        std::process::exit(0);
    }

    // `trim()` removes the newline, and we lowercase everything so that
    // "Rust" and "RUST" both match the secret word.
    let text = input.trim().to_ascii_lowercase();

    let letter_count = text.chars().count();
    if letter_count == 0 || !text.chars().all(|c| c.is_alphabetic()) {
        return None;
    }

    if letter_count == 1 {
        // Safe to unwrap: we just checked there is exactly one character.
        Some(Guess::Letter(text.chars().next().expect("one char")))
    } else {
        Some(Guess::Word(text))
    }
}
