use rand::Rng;
use std::collections::HashSet;
use std::io;

fn main() {
    println!("Welcome to Hangman!");

    // Word list (using Vec)
    let words = vec!["rust", "ownership", "variable", "memory", "threading"];
    let secret_word = words[rand::thread_rng().gen_range(0..words.len())]; // Randomly select a word
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut attempts_left = 6;

    println!("The secret word has {} letters.", secret_word.len());

    // Main game loop
    while attempts_left > 0 {
        // Display current state
        let display_word: String = secret_word
            .chars()
            .map(|c| {
                if guessed_letters.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        println!("Word: {}", display_word);
        println!("Attempts left: {}", attempts_left);

        // Check if the user has won
        if !display_word.contains('_') {
            println!("Congratulations! You guessed the word: {}", secret_word);
            return;
        }

        // Ask the player to guess a letter
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess = guess.trim().chars().next().unwrap_or(' ');

        // Process the guess
        if guessed_letters.contains(&guess) {
            println!("You already guessed '{}'. Try a different letter.", guess);
        } else if secret_word.contains(guess) {
            println!("Good guess! '{}' is in the word.", guess);
            guessed_letters.insert(guess);
        } else {
            println!("Sorry, '{}' is not in the word.", guess);
            guessed_letters.insert(guess);
            attempts_left -= 1;
        }
    }

    // Game over
    println!("Game over! The word was '{}'. Better luck next time!", secret_word);
}
