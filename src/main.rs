extern crate core;
use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::os::raw::c_char;

use std::{thread::sleep, time::Duration};

pub fn main() {
    let discovered: char = '□';
    let cursor: char = '⛶';
    let undiscovered: char = '■';
    let flag: char = '◩';
    //  let mut cursor1:char = '⬚';


    let mut course: [[char; 30]; 16] = [['a'; 30]; 16];

    for x in 0..16 {
        for y in 0..30 {
            course[x][y] = undiscovered;
            course[5][5] = cursor;
            print!("  {}", course[x][y]);
        }
        println!();
    }



    // Rapidfire for videogames.


    // Create a handler to trigger on any and all keyboard events.
    WKey.bind(|| println!("Up"));
    SKey.bind(|| println!("Down"));
    DKey.bind(|| println!("Right"));
    AKey.bind(|| println!("Left"));
    QKey.bind(|| MouseCursor::move_rel(10, 10));
    handle_input_events();
    loop {

        if AKey.is_toggled() {

            print!("done");
            break;
        }
    }



    // Call this to start listening for bound inputs.
    handle_input_events();

}