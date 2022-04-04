use crossterm::{cursor, QueueableCommand};

extern crate crossterm;

pub fn overwrite(message: String) {
    std::io::stdout().queue(cursor::MoveUp(1 as u16)).unwrap();
    println!("{}", message);
}