extern crate core;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::os::raw::c_char;
use std::{thread::sleep, time::Duration};
use std::io::Cursor;

use colored::Colorize;
use rand::Rng;

const discovered: char = '□';
const cursor: char = '⛶';
const undiscovered: char = '■';
const flag: char = '◩';
static mut cursor_x: usize = 0;
static mut cursor_y: usize = 0;
static mut course: [[char; 30]; 16] = [[undiscovered;30];16];
static mut mines: [[i8;30];16]= [[-1;30];16];


fn PrettyPrint(courses: [[char; 30]; 16], cx: usize, cy: usize) {
    //you can use a control character


        print!("{}[2J", 27 as char);


    for x in 0..16 {
        for y in 0..30 {
            if cx == x && cy == y {
                print!("  {}", format!("{}",cursor).yellow());
            } else if courses[x][y] == flag{
                print!("  {}", format!("{}",flag).red())
            }
            else{
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

unsafe fn lose(){

    for x in 0..16 {
        for y in 0..30 {
            if mines[x][y]==1 {
                print!("  {}", format!("{}",'*').red());
            } else {
                print!("  {}", format!("{}",discovered).black())
            }

        }
        println!();
    }
    std::process::exit(0);
}

unsafe fn openField() {
    if course[cursor_x][cursor_y] == undiscovered && mines[cursor_x][cursor_y]==1{
        course[cursor_x][cursor_y] = '*';
        lose();
    } else if course[cursor_x][cursor_y] == undiscovered && mines[cursor_x][cursor_y]==-1{
        course[cursor_x][cursor_y] = discovered;
    }
    PrettyPrint(course,cursor_x,cursor_y);
}

  unsafe fn generateMines() {
      let mut mineNum: i16 = 100;
      let mut rng = rand::thread_rng();
      while mineNum != 0 {
          let x: usize = rng.gen::<usize>() % 16;
          let y: usize = rng.gen::<usize>() % 30;
          if mines[x][y] == -1 {
              mines[x][y] = 1;
              mineNum = mineNum - 1;
          }
      }
  }



fn main() {

    //  let mut cursor1:char = '⬚';

    //unsafe { course = generateCourse(); }



    unsafe {
        generateMines();
        WKey.bind(move || {
            subX();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        SKey.bind(move || {
            addX();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        DKey.bind(move || {
            addY();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        AKey.bind(move || {
            subY();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        EnterKey.bind(move|| { setFlag(); PrettyPrint(course,cursor_x,cursor_y) } );
        SpaceKey.bind(move || {openField();});
        QKey.bind(|| MouseCursor::move_rel(10, 10));
    }
    loop{
    handle_input_events();}
    println!("HERE");
}
