use std::io::{self, stdout, Write};
use std::env;
use termion::raw::IntoRawMode;
use termion::{ event::Key, input::TermRead };
use crate::terminal::get_terminal_size;
use crate::viewer::{ Document, set_startup_document};

#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    exit: bool,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            exit: false,
            cursor_position: Position::default(),
            offset: Position::default(),
            document: set_startup_document(),
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        Editor::clear_screen();
        Editor::cursor_position(&self.cursor_position);

        loop {
            if let Err(e) = self.refesh_screen() {
                Editor::dead(&e);
            }
            if let Err(e) = self.process_keypress() {
                Editor::dead(&e);
            }
            if self.exit {
                break;
            }
        }
    }

    fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    fn refesh_screen(&self) -> Result<(), std::io::Error> {
        Editor::hide_cursor();
        if self.exit {
            Editor::clear_screen();
        } else {
            draw_rows();
            self.draw_content(&self.document);
            Editor::cursor_position(&self.cursor_position);
        }
        Editor::show_cursor();
        Editor::flush()
    }

    fn draw_content(&self, document: &Document) {
        let last_line = get_terminal_size().unwrap().height as usize + self.offset.y;
        let terminal_width = get_terminal_size().unwrap().width as usize;

        for (index_line, line) in (self.offset.y..last_line).enumerate() {
            Editor::cursor_position(&Position {x: 0, y: index_line});
            print!("{}\r", match document.rows.get(line) {
                Some(line) => {
                    if line.len() > terminal_width {
                        let last_char = if terminal_width + self.offset.x > line.len() { line.len() } else { terminal_width + self.offset.x};

                        &line[self.offset.x..last_char]
                    } else if line.len() < self.offset.x {
                        ""
                    } else {
                        &line[self.offset.x..]
                    }
                }
                None => break,
            });
        }
    }

    fn dead(e: &std::io::Error) {
        Editor::clear_screen();
        panic!("{}", e);
    }

    fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    fn cursor_position(position: &Position) {
        print!("{}", termion::cursor::Goto(position.x.saturating_add(1) as u16, position.y.saturating_add(1) as u16));
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('c') => {
                self.exit = true;
            }
            Key::Left | Key::Right | Key::Up | Key::Down |
            Key::Home | Key::End | Key::PageUp | Key::PageDown => {
                Editor::moving_cursor(pressed_key, &mut self.cursor_position, &self.document, &mut self.offset);
            }
            _ => (),
        }
        Ok(())
    }

    fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    fn moving_cursor(pressed_key: Key, cursor_position: &mut Position, document: &Document, offset: &mut Position) {
        let mut position = Position {
            x: cursor_position.x,
            y: cursor_position.y,
        };

        let max_height = get_terminal_size().expect("can't get terminal height").height as usize - 1;
        let max_width  = get_terminal_size().expect("can't get terminal width").width   as usize - 1;

        let current_line: String = match document.rows.get(offset.y + cursor_position.y) {
            Some(line) => line.clone(),
            None => String::new(),
        };

        match pressed_key {
            Key::Up => {
                position.y = position.y.saturating_sub(1);
                if offset.y > 0 && cursor_position.y == 0 {
                    offset.y = offset.y.saturating_sub(1);
                }
            }
            Key::Down => {
                if position.y < max_height - 1 {
                    position.y = position.y.saturating_add(1);
                } else if offset.y < document.rows.len().saturating_sub(max_height) {
                    offset.y = offset.y.saturating_add(1);
                }
            }
            Key::Left => {
                position.x = position.x.saturating_sub(1);
                if cursor_position.x == 0 && offset.x != 0 {
                    offset.x -= 1;
                }
            }
            Key::Right => {
                if position.x < max_width {
                    position.x = position.x.saturating_add(1);
                }
                if cursor_position.x == max_width &&
                    current_line.len() > max_width &&
                    (offset.x + max_width + 1) != current_line.len() {
                    offset.x = offset.x.saturating_add(1);
                }
            }
            Key::Home => {
                position.x = 0;
                offset.x = 0;
            }
            Key::End => {
                position.x = current_line.len() - 1 - offset.x;
                if current_line.len() > max_width + 1 {
                    position.x = max_width - 1;
                    offset.x = current_line.len() - max_width;
                }
            }
            Key::PageUp => position.y = 0,
            Key::PageDown => position.y = max_height - 1,
            _ => (),
        }

        cursor_position.x = position.x;
        cursor_position.y = position.y;

    }

    fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}

fn draw_rows() {
    let args: Vec<String>  = env::args().collect();
    // In debug state
    let max_width  = String::from("four").len();
    for rows in 0..get_terminal_size().expect("Can't get height of terminal").height {
        Editor::clear_current_line();
        if args.get(1) == None && rows == 2 {
            draw_welcome_message();
            continue;
        }
        println!("{}\r", max_width);
    }
}

fn draw_welcome_message() {
        let welcome_message = format!("~ Semplice version {} \r", env!("CARGO_PKG_VERSION"));
        let width = std::cmp::min(welcome_message.len(), get_terminal_size().expect("Can't get terminal size").width as usize);
        println!("{}", &welcome_message[..width]);

}

fn read_key() -> Result<Key, std::io::Error> {
   loop {
       if let Some(key) = io::stdin().lock().keys().next() {
           return key;
       }
    }
} 
