use std::io;
use std::io::Read;
use crossterm::terminal;

struct Cleaner;

impl Drop for Cleaner {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

/*
 * =========
 * EDIT
 * =========
 * A simple text editor. https://medium.com/@otukof/build-your-text-editor-with-rust-678a463f968b
 */
pub fn edit() {
    let _cleaner = Cleaner;
    terminal::enable_raw_mode().expect("error: cannot enable Crossterm raw mode");

    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("error: cannot read line") == 1 {

    }
}