//! The ASCII art for the gallows.
//!
//! Keeping the art in its own file means `main.rs` stays focused on the
//! game logic, and this file is just pictures -- easy to edit or redraw.
//!
//! In Rust, a file inside `src/` becomes a *module*. `main.rs` pulls it in
//! with `mod gallows;` and then reaches the items here as `gallows::...`.

/// How many pictures we have: one for each number of mistakes, 0 through 6.
pub const STAGES: usize = 7;

/// One picture per number of mistakes, from 0 (empty gallows) to 6 (full).
///
/// These use *raw strings* (`r"..."`), which take every character literally.
/// That is handy for art with backslashes in it. The trade-off is that the
/// text starts right where `r"` ends, so each line of art must begin at the
/// very start of the line -- no indentation allowed.
const GALLOWS: [&str; STAGES] = [
    r"  +---+
  |   |
      |
      |
      |
      |
=========",
    r"  +---+
  |   |
  O   |
      |
      |
      |
=========",
    r"  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
    r"  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
    r"  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========",
    r"  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========",
    r"  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========",
];

/// Return the picture for a given number of mistakes.
///
/// `wrong` is clamped so that asking for too many mistakes returns the final
/// picture instead of panicking. `pub` makes this callable from `main.rs`.
pub fn for_mistakes(wrong: usize) -> &'static str {
    let index = wrong.min(STAGES - 1);
    GALLOWS[index]
}
