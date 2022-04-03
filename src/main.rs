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
static mut course: [[char; 30]; 16] = [[undiscovered; 30]; 16];
static mut mines: [[usize; 30]; 16] = [[0; 30]; 16];
const numbers: [char; 9] = [discovered, '1', '2', '3', '4', '5', '6', '7', '8'];


fn PrettyPrint(courses: [[char; 30]; 16], cx: usize, cy: usize) {
    //you can use a control character


    print!("{}[2J", 27 as char);


    for x in 0..16 {
        for y in 0..30 {
            if cx == x && cy == y {
                print!("  {}", format!("{}", cursor).yellow());
            } else if courses[x][y] == flag {
                print!("  {}", format!("{}", flag).red())
            } else if courses[x][y] == undiscovered {
                print!("  {}", courses[x][y]);
            } else {
                print!("  {}", courses[x][y]);
                /* match courses[x][y]{

                     numbers[1] =>{  print!("  {}", format!("{}",numbers[1]).cyan());}
                     numbers[2] =>{  print!("  {}", format!("{}",numbers[2]).green());}
                     numbers[3] =>{  print!("  {}", format!("{}",numbers[3]).magenta());}
                     numbers[4] =>{  print!("  {}", format!("{}",numbers[4]).blue());}
                     numbers[5] =>{  print!("  {}", format!("{}",numbers[5]).red());}
                     numbers[6] =>{  print!("  {}", format!("{}",numbers[6]).yellow());}
                     numbers[7] =>{  print!("  {}", format!("{}",numbers[7]).white());}
                     _ => {} }

                      */
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
    if course[cursor_x][cursor_y] == undiscovered {
        course[cursor_x][cursor_y] = flag;
    } else if course[cursor_x][cursor_y] == flag {
        course[cursor_x][cursor_y] = undiscovered;
    }
}

unsafe fn lose() {
    for x in 0..16 {
        for y in 0..30 {
            if mines[x][y] == 1 {
                print!("  {}", format!("{}", '*').red());
            } else {
                print!("  {}", format!("{}", discovered).black())
            }
        }
        println!();
    }
    std::process::exit(0);
}

unsafe fn openField() {
    if course[cursor_x][cursor_y] == undiscovered && mines[cursor_x][cursor_y] == 1 {
        course[cursor_x][cursor_y] = '*';
        lose();
    } else if course[cursor_x][cursor_y] == undiscovered && mines[cursor_x][cursor_y] == 0 {
        course[cursor_x][cursor_y] = discovered;
        let mut num: usize = 0;
        if cursor_y == 0 && cursor_x == 0 {
            num = num + mines[cursor_x + 1][cursor_y + 1];
            num = num + mines[cursor_x][cursor_y + 1];
            num = num + mines[cursor_x + 1][cursor_y];
        }
        else if cursor_y == 29 && cursor_x == 15 {
            num = num + mines[cursor_x - 1][cursor_y - 1];
            num = num + mines[cursor_x][cursor_y - 1];
            num = num + mines[cursor_x - 1][cursor_y];
        }

        else if cursor_y == 29 && cursor_x == 0 {
            num = num + mines[cursor_x + 1][cursor_y - 1];
            num = num + mines[cursor_x][cursor_y - 1];
            num = num + mines[cursor_x + 1][cursor_y];
        }
        else if cursor_y == 0 && cursor_x == 15 {
            num = num + mines[cursor_x - 1][cursor_y + 1];
            num = num + mines[cursor_x][cursor_y + 1];
            num = num + mines[cursor_x - 1][cursor_y];
        }
        else if cursor_y == 29 {
            num = num + mines[cursor_x + 1][cursor_y];
            num = num + mines[cursor_x - 1][cursor_y - 1];
            num = num + mines[cursor_x + 1][cursor_y - 1];
            num = num + mines[cursor_x][cursor_y - 1];
            num = num + mines[cursor_x - 1][cursor_y];
        }
        else if cursor_x == 0 {
            num = num + mines[cursor_x + 1][cursor_y + 1];
            num = num + mines[cursor_x][cursor_y + 1];
            num = num + mines[cursor_x + 1][cursor_y - 1];
            num = num + mines[cursor_x][cursor_y - 1];
            num = num + mines[cursor_x + 1][cursor_y];
        }
        else if cursor_x == 15 {
            num = num + mines[cursor_x - 1][cursor_y + 1];
            num = num + mines[cursor_x][cursor_y + 1];
            num = num + mines[cursor_x - 1][cursor_y - 1];
            num = num + mines[cursor_x][cursor_y - 1];
            num = num + mines[cursor_x - 1][cursor_y];
        }
        else if cursor_y==0 {
            num = num + mines[cursor_x + 1][cursor_y];
            num = num + mines[cursor_x - 1][cursor_y + 1];
            num = num + mines[cursor_x + 1][cursor_y + 1];
            num = num + mines[cursor_x][cursor_y + 1];
            num = num + mines[cursor_x - 1][cursor_y];
        }
        else {
            num = num + mines[cursor_x + 1][cursor_y+1];
            num = num + mines[cursor_x - 1][cursor_y-1];
            num = num + mines[cursor_x - 1][cursor_y+1];
            num = num + mines[cursor_x + 1][cursor_y-1];
            num = num + mines[cursor_x][cursor_y+1];
            num = num + mines[cursor_x][cursor_y-1];
            num = num + mines[cursor_x - 1][cursor_y];
            num = num + mines[cursor_x + 1][cursor_y];
        }
        course[cursor_x][cursor_y] = numbers[num];
    }
    PrettyPrint(course, cursor_x, cursor_y);
}

unsafe fn generateMines() {
    let mut mineNum: i16 = 100;
    let mut rng = rand::thread_rng();
    while mineNum != 0 {
        let x: usize = rng.gen::<usize>() % 16;
        let y: usize = rng.gen::<usize>() % 30;
        if mines[x][y] == 0 {
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
        EnterKey.bind(move || {
            setFlag();
            PrettyPrint(course, cursor_x, cursor_y)
        });
        SpaceKey.bind(move || { openField(); });
        QKey.bind(|| MouseCursor::move_rel(10, 10));
    }
    loop {
        handle_input_events();
    }
    println!("HERE");
}
