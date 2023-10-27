extern crate rand;
extern crate crossterm;

use rand::seq::SliceRandom;
use std::error::Error;
use crossterm::event::{self};
use std::io::{self, Write}; // Add Write trait

fn fetch_wordlist() -> Vec<String> {
    include_str!("words.txt").lines().map(|line| line.to_string()).collect()
}

fn wait_for_keypress() {
    loop {
        if let Ok(event) = event::read() {
            if let crossterm::event::Event::Key(_) = event {
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let words_pool = fetch_wordlist();

    println!("Press any key to get a word. CTRL+C to exit.\n");

    loop {
        wait_for_keypress();

        if let Some(word) = words_pool.choose(&mut rand::thread_rng()) {
            print!("{} ", word);
            io::stdout().flush()?;
        } else {
            println!("No words found in the list.");
            break;
        }
    }

    Ok(())
}
