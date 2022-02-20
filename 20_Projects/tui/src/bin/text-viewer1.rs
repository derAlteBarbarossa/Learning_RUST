use std::env;
use std::fs;
use std::io;
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
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

    fn set_pos(&mut self, x: usize, y:usize) {
        self.cursor_position.x = x;
        self.cursor_position.y = y;

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_position.x as u16, 
                                self.cursor_position.y as u16)
        )
    }

    pub fn show_document(&mut self) {
        let (old_x, old_y) = (self.cursor_position.x, self.cursor_position.y);
       
        print!("{}{}", termion::clear::All,
                termion::cursor::Goto(1,1));

        println!(
            "{}{} Welcome to Super TextEditor\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );

        for line in &self.document.lines {
            println!("{}\r", line);
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y) as u16)
        );

        println!(
            "{}{} line-count: {} Filename: {}{}\r{}{}",
            color::Fg(color::Red),
            style::Bold,
            self.document_length,
            self.file_name,
            style::Reset,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        );

        self.set_pos(old_x, old_y);
    }

    pub fn run(&mut self) {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let stdin = io::stdin();

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }

                _ => {}
            }
            stdout.flush().unwrap();
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
    //TextViewer::run();
}