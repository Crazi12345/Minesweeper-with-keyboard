extern crate core;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::os::raw::c_char;
use std::{thread::sleep, time::Duration};
use std::io::Cursor;

const discovered: char = '□';
const cursor: char = '⛶';
const undiscovered: char = '■';
const flag: char = '◩';
static mut cursor_x: usize = 0;
static mut cursor_y: usize = 0;
static mut course: [[char; 30]; 16] = [[undiscovered;30];16];


fn PrettyPrint(courses: [[char; 30]; 16], cx: usize, cy: usize) {
    for x in 0..16 {
        for y in 0..30 {
            if cx == x && cy == y {
                print!("  {}", cursor);
            } else {
                print!("  {}", courses[x][y]);
            }
        }
        println!();
    }
}

unsafe fn SafeCallPrettyPrint(coursea: [[char; 30]; 16], x: usize, y: usize) {
    PrettyPrint(coursea, x, y);
}

unsafe fn addX() {
    if cursor_x != 15 {
        cursor_x = cursor_x + 1;
    }
}

unsafe fn subX() {
    if cursor_x != 0 {
        cursor_x = cursor_x - 1;
    }
}

unsafe fn addY() {
    if cursor_y != 29 {
        cursor_y = cursor_y + 1;
    }
}

unsafe fn subY() {
    if cursor_y != 0 {
        cursor_y = cursor_y - 1;
    }
}

unsafe fn setFlag() {
    if course[cursor_x][cursor_y] == undiscovered{
        course[cursor_x][cursor_y]= flag;}
    else if course[cursor_x][cursor_y] == flag{
        course[cursor_x][cursor_y] = undiscovered;
    }


}
/*
  fn generateCourse() -> [[char; 30]; 16] {
    let mut courseTemp: [[char; 30]; 16]=[[undiscovered;30];16] ;
    for x in 0..16 {
        for y in 0..30 {
            courseTemp[x][y] = undiscovered;

            print!("  {}", courseTemp[x][y]);
        }
        println!();
    }
    courseTemp[5][0] = '5';
    return courseTemp;
}*/

fn main() {

    //  let mut cursor1:char = '⬚';

    //unsafe { course = generateCourse(); }



    unsafe {
        WKey.bind(move || {
            subX();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        SKey.bind(move || {
            addX();
            SafeCallPrettyPrint(course, cursor_x, cursor_y)
        });
        DKey.bind(move || {
            addY();
            SafeCallPrettyPrint(course, cursor_x, cursor_y)
        });
        AKey.bind(move || {
            subY();
            SafeCallPrettyPrint(course, cursor_x, cursor_y)
        });
        EnterKey.bind(move|| { setFlag(); PrettyPrint(course,cursor_x,cursor_y) } );
        QKey.bind(|| MouseCursor::move_rel(10, 10));
    }
    handle_input_events();
    println!("HERE");
}
