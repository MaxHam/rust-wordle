use std::{char::MAX, io};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fs::{read, File};
use std::io::{BufRead, BufReader, Result};

const MAX_ATTEMPTS: i32 = 5;

fn read_word_file(path: String) -> Result<Vec<String>> {
    // Open the file for reading
    let file: File = File::open(path)?;

    // Create a buffered reader to read the file
    let buffered: BufReader<File> = BufReader::new(file);

    // Create a vector to hold the words.
    let mut words: Vec<String> = Vec::new();

    // Read the file line by line using the lines() iterator from BufReader.
    for line in buffered.lines() {
        // For each line, which is returned as a Result<String>, add it to the vector.
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
    println!("------------------------------");

    let mut attempt: i32 = 1;

    while attempt <= MAX_ATTEMPTS {
        println!("Guess {attempt}/{MAX_ATTEMPTS}");

        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: String = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        if guess == secret_word {
            println!("You guessed right. You win!");
            break
        } else {
            println!("You guessed wrong.");
        }
        attempt = attempt + 1;
    }
}