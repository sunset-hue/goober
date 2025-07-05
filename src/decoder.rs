use std::io::{stdout, Write};

use crossterm;

pub fn terminal_support_ansi() -> bool {
    crossterm::ansi_support::supports_ansi()
}

pub fn raw_code_to_character(eventcharacter: &crossterm::event::KeyEvent) {
    let mut output = stdout();
    let character_code = eventcharacter.code.as_char().unwrap_or('f');
    // super not convienient but ok ig

    output.write_all(character_code.to_string().as_bytes()).expect("could not decode idk");
    // testing needed

}

