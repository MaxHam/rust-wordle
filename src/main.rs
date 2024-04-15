use std::io;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const MAX_ATTEMPTS: i32 = 5;

fn count_characters(input: &str) -> usize {
    input.chars().count()
}

fn read_word_file(path: String) -> Result<Vec<String>> {
    let file: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();

    for line in buffered.lines() {
        let line: String = line?;
        words.push(line);
    }

    Ok(words)
}

fn pick_word(words: Result<Vec<String>>) -> String{
   match words {
       Ok(words) => {
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..words.len());
        let random_word = &words[random_index];
        return String::from(random_word)
       }
       Err(e) => {
        eprintln!("Failed to read file: {}", e);
        return String::from("No word found")
       }
   }
}

fn main() {
    let words: Result<Vec<String>> = read_word_file(String::from("words.txt"));
    let secret_word: String = pick_word(words);

    println!("----------- Wordle -----------");
    println!("The secret word is '{secret_word}'");
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

        if guess == secret_word {
            println!("You guessed right. You win!");
            break
        } else {
            println!("You guessed wrong.");
        }
        attempt = attempt + 1;
    }
    println!("You ran out of attempts. Try again.");
}