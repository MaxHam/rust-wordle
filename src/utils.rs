use colored::Colorize;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
pub enum Status {
    GREEN = 1,
    YELLOW = 0,
    RED = -1,
    GREY = -2,
}
#[derive(Debug)]
pub struct Letter {
    pub character: char,
    pub status: Status,
}

pub fn print_letter(letter: &Letter) {
    match letter.status {
        Status::RED => {
            print!("{} ", letter.character.to_string().red())
        }
        Status::GREEN => {
            print!("{} ", letter.character.to_string().green())
        }
        Status::GREY => {
            print!("{} ", letter.character.to_string().white())
        }
        Status::YELLOW => {
            print!("{} ", letter.character.to_string().yellow())
        }
    }
}

pub fn parse_chars_to_letters(chars: Vec<char>) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();
    for char in chars {
        letters.push(Letter {
            character: char,
            status: Status::GREY,
        });
    }

    letters
}

pub fn count_characters(input: &str) -> usize {
    input.chars().count()
}

pub fn split_string(string: &str) -> Vec<char> {
    string.chars().collect()
}

pub fn read_word_file(path: String) -> Result<Vec<String>> {
    let file: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();

    for line in buffered.lines() {
        let line: String = line?;
        words.push(line);
    }

    Ok(words)
}

pub fn pick_word(words: Result<Vec<String>>) -> String {
    match words {
        Ok(words) => {
            let mut rng = thread_rng();
            let random_index = rng.gen_range(0..words.len());
            let random_word = &words[random_index];
            return String::from(random_word);
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return String::from("No word found");
        }
    }
}
