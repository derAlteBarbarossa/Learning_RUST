use std::env;
use std::fs;
//use std::io;
use std::io::{self, Write};

use termion::event::*;
use termion::input::{TermRead, MouseTerminal};
use termion::cursor::{self, DetectCursorPos};
use termion::raw::IntoRawMode;
use termion::{color, style};

struct Doc {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

struct TextViewer {
    document:               Doc,
    document_length:        usize,

    cursor_position:        Coordinates,
    terminal_size:          Coordinates,

    file_name:              String
}

impl TextViewer {
    // Private methods
    fn set_pos(&mut self, x: usize, y:usize) {
        self.cursor_position.x = x;
        self.cursor_position.y = y;

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16, 
                                self.cursor_position.y as u16)
        )
    }

    fn inc_x(&mut self) {
        if self.cursor_position.x < self.terminal_size.x {
            self.cursor_position.x += 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16,
                self.cursor_position.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if self.cursor_position.x > 1 {
            self.cursor_position.x -= 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16,
                self.cursor_position.y as u16)
        );
    }


    fn inc_y(&mut self) {
        if self.cursor_position.y < self.document_length {
            self.cursor_position.y += 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16,
                self.cursor_position.y as u16)
        );
    }

    fn dec_y(&mut self) {
        if self.cursor_position.y > 1 {
            self.cursor_position.y -= 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16,
                self.cursor_position.y as u16)
        );
    }
    // Public Interface
    pub fn init(file_name: &str) -> Self {
        let mut document = Doc {
            lines: vec![],
        };

        let file_handle = fs::read_to_string(file_name).unwrap();

        for line in file_handle.lines() {
            document.lines.push(line.to_string());
        }

        let document_length = file_handle.lines().count();

        let size = termion::terminal_size().unwrap();

        TextViewer {
            document:           document,
            document_length:    document_length,

            cursor_position:    Coordinates {
                x: 1, 
                y: document_length
            },

            terminal_size:      Coordinates {
                x: size.0 as usize,
                y: size.1 as usize
            },

            file_name:          file_name.to_string(),
        }
    }


    pub fn show_document(&mut self) {
        //let (old_x, old_y) = (self.cursor_position.x, self.cursor_position.y);
       
        print!("{}{}", termion::clear::All,
                termion::cursor::Goto(1,1));

        println!(
            "{}{} Welcome to Super TextEditor\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );

        if self.document_length < self.terminal_size.y {
            for line in 0..self.document_length {
                println!("{}\r", self.document.lines[line as usize]);
            }
        } else {
            if self.cursor_position.y <= self.terminal_size.y {
                for line in 0..self.terminal_size.y - 3 {
                    println!("{}\r", self.document.lines[line as usize]);
                }
            } else {
                for line in self.cursor_position.y - (self.terminal_size.y - 3)
                    ..self.cursor_position.y {
                    println!("{}\r", self.document.lines[line as usize]);
                }
            }
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16)
        );

        println!(
            "{}{} X = {}, Y = {}, line-count: {} Filename: {}{}\r{}{}",
            color::Fg(color::Red),
            style::Bold,
            self.cursor_position.x,
            self.cursor_position.y,
            self.document_length,
            self.file_name,
            style::Reset,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        );

        self.set_pos(self.cursor_position.x, self.cursor_position.y)
    }

    pub fn run(&mut self) {
        let mut stdout_mouse = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

        let stdin = io::stdin();

        for c in stdin.events() {
            let event = c.unwrap();

            match event {
                Event::Key(Key::Ctrl('q')) => {
                    self.set_pos(0, self.terminal_size.y);
                    self.show_document();
                    println!(
                        "\r{}{}",
                        color::Fg(color::Reset),
                        color::Bg(color::Reset)
                    );
                    break;
                }
                Event::Key(Key::Left) => {
                    self.dec_x();
                    self.show_document(); 
                }
                Event::Key(Key::Right) => {
                    self.inc_x();
                    self.show_document();
                }
                Event::Key(Key::Down) => {
                    self.inc_y();
                    self.show_document(); 
                }
                Event::Key(Key::Up) => {
                    self.dec_y();
                    self.show_document();
                }
                Event::Mouse(m) => match m {
                    MouseEvent::Release(a,b) |
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Hold(a, b) => {
                        write!(stdout_mouse, "{}", cursor::Goto(a, b)).unwrap();
                        let (x, y) = stdout_mouse.cursor_pos().unwrap();
                        self.set_pos(x as usize, y as usize);
                        self.show_document();
                    }
                }

                _ => {}
            }
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    // If no file name was given
    if args.len() < 2 {
        println!("Please provide file name as argument");
        std::process::exit(0);
    }

    // Check if the input file exists
    if !std::path::Path::new(&args[1]).exists() {
        println!("File {} doesn't exists!", args[1]);
        std::process::exit(0);
    }

    println!("{}", termion::cursor::Show);

    // Initialise the TextViewer
    let mut viewer = TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}