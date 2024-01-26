mod gui;
use crate::gui::gui::launcher;

fn main() {
    match launcher() {
        Ok(_) => println!("Success"),
        Err(e) => println!("{}", e.to_string())
    }
}