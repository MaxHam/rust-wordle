use colored::Colorize;

use crate::utils::{
    count_characters, parse_chars_to_letters, pick_word, print_letter, read_word_file,
    split_string, Letter, Status,
};
use std::io;
use std::io::Result;

mod utils;

const MAX_ATTEMPTS: i32 = 5;

fn main() -> Result<()> {
    let words: Result<Vec<String>> = read_word_file(String::from("words.txt"));
    let secret_word: String = pick_word(words);
    let chars_secret_word: Vec<char> = split_string(&secret_word);
    let mut guesses: Vec<Vec<Letter>> = vec![];

    println!("----------- Wordle -----------\n");
    println!(
        "You have {} attempts to guess the secret german word with 5 letters.
If you guess the exact position of a letter it is printed {}.
If the guessed letter is not at the exact position but somewhere in the secret word it appears {}.
If the letter is not part of the secret word at all it is printed {}.\n",
        MAX_ATTEMPTS,
        "green".green(),
        "yellow".yellow(),
        "red".red()
    );
    println!("Guess the word!");

    let mut attempt: i32 = 1;

    while attempt <= MAX_ATTEMPTS {
        println!("--------------- {attempt}/{MAX_ATTEMPTS} ---------------");

        let mut guess: String = String::new();

        loop {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            guess = match guess.trim().to_lowercase().parse() {
                Ok(guess) => guess,
                Err(_) => continue,
            };

            let len_guess = count_characters(&guess);

            // wordle only contains 5 character words
            if len_guess == 5 {
                break;
            }

            guess.clear();
            println!("Your word has {len_guess} characters. Type a word with 5 characters.");
        }

        let chars_guess: Vec<char> = split_string(&guess);
        let mut letters_guess: Vec<Letter> = parse_chars_to_letters(chars_guess);

        /*
        Iterate all characters of valid guess and check whether the guessed character is
        either
        - at the correct position
        - somewhere in the word
        - or is not part of the secred word
        (could be done with custom match statement)
            */
        for (pos, letter) in letters_guess.iter_mut().enumerate() {
            if letter.character == chars_secret_word[pos] {
                letter.status = Status::GREEN;
            } else if chars_secret_word.contains(&letter.character) {
                letter.status = Status::YELLOW;
            } else {
                letter.status = Status::RED;
            }
        }

        guesses.push(letters_guess);

        for (pos, guess) in guesses.iter().enumerate() {
            print!("{}: ", pos + 1);
            for letter in guess {
                print_letter(letter)
            }
            // force new line
            println!();
        }

        if guess == secret_word {
            println!("You guessed right. You win!");
            return Ok(());
        }
        attempt = attempt + 1;
    }
    println!(
        "You ran out of attempts. The secret word was {}. Try again.",
        secret_word.green()
    );
    return Ok(());
}
