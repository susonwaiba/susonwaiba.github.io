use crate::about::build_about_json;
use crate::note::build_note_json;

mod about;
mod file;
mod note;

fn main() {
    build_note_json();
    build_about_json();
    println!("\n");
    println!("Completed!");
}
