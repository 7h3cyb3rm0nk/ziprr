use std::env;
pub mod worker;
pub mod reader;
pub mod finder;
use crate::finder::password_finder;
fn main() {
    let zip_path = env::args().nth(1).unwrap();
    let dictionary_path = "/home/ramees/Downloads/puzzle/wooden_door_files/wordlist.txt";
    let workers = 3;
    password_finder(&zip_path, dictionary_path, workers);
}
