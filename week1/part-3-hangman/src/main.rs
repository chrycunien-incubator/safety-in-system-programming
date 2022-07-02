// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn vec_to_string(pool: Vec<char>) -> String {
    pool.into_iter().map(|i| i.to_string()).collect::<String>()
}

fn print_status(secret_word : Vec<char>, pool: Vec<char>) {
    let mut v: Vec<char> = Vec::new();
    for i in secret_word.iter() {
        if pool.contains(&i) {
            v.push(*i);
        } else {
            v.push('-')
        }
    }
    println!("The word so far is {}", vec_to_string(v))
}

fn print_letters(pool: Vec<char>) {
    println!("You have guessed the following letters: {}", vec_to_string(pool))
}

fn print_win_message(s: String) {
    println!("Congratulations you guessed the secret word: {}!", s)
}

fn is_win(secret_word : Vec<char>, pool: Vec<char>) -> bool {
    for i in secret_word.iter() {
        if !pool.contains(&i) {
            return false
        } 
    }
    true
}


fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut counter = NUM_INCORRECT_GUESSES;
    let mut guesses = Vec::new();
    while counter > 0 {
        print_status(secret_word_chars.clone(), guesses.clone());
        print_letters(guesses.clone());
        println!("You have {} guesses left", counter);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let letter = guess.chars().nth(0).unwrap();
        if !secret_word_chars.contains(&letter) || guesses.contains(&letter) {
            counter -= 1;
        }
        guesses.push(letter.clone());
        println!();
        if is_win(secret_word_chars.clone(), guesses.clone()) {
            print_win_message(secret_word.clone());
            return
        }
        
    }
    println!("Sorry, you ran out of guesses!")
}
