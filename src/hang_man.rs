use std::fs;
use std::io::{BufRead, Write};

#[derive(Debug)]
struct HangMan {
    correct_word: String,
    user_guess: String,
    guesses_allowed: usize,
}

impl HangMan {
    /// Create a new HangMan instance and sets the correct word
    /// # returns
    /// a new hangman instance
    /// # Example:
    /// ```
    /// let hangman = HangMan::new();
    /// ```
    pub fn new() -> HangMan {
        let mut hang_man = HangMan {
            correct_word: String::new(),
            user_guess: String::new(),
            guesses_allowed: 0,
        };
        hang_man.get_answer();
        return hang_man;
    }

    /// reads the currect word from answer file, and return the hangman struct
    /// # Arguments
    /// * `hangman` - the hangman struct to store the correct word
    /// # Returns
    /// * `hangman` - the hangman struct with the correct word added
    /// # Example
    /// ```
    /// let hangman = HangMan::new();
    /// hangman.get_answer()
    /// ```
    pub fn get_answer(&mut self) {
        let answer = fs::read_to_string("./src/answer.txt");
        self.correct_word = answer.unwrap();

        // println!(
        // "length: {}, correct word: {}",
        // self.correct_word.len(),
        // self.correct_word
        // );

        for index in 0..self.correct_word.len() - 1 {
            println!("index {}", index);
            println!("user guess: {}", self.user_guess);
            self.user_guess.push_str("_");
        }
    }

    /// asks user for number of guesses allowed
    ///
    /// * `reader`: Something that mocks stdin
    /// * `writer`: Something that mocks stdout
    pub fn game_start_details<R, W>(&mut self, mut reader: R, mut writer: W)
    where
        R: BufRead,
        W: Write,
    {
        println!("get start details");
        write!(&mut writer, "{}", "Number of guesses allowed: ").expect("Unable to write");
        let mut input = String::new();
        let mut valid: bool = false;
        while !valid {
            // get user input
            match reader.read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim().parse::<usize>().unwrap();
                    // make sure the guess is larger than the length of correct word
                    println!("{}", self.correct_word.len());
                    valid = true; // NOTE: temp fix
                                  // if input <= self.correct_word.len() {
                                  // self.guesses_allowed = input;
                                  // valid = true;
                                  // }
                }
                Err(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test if the answer is read correctly from file
    fn get_answer_test() {
        let mut hangman = HangMan::new();
        assert_eq!(hangman.correct_word, "apple\n");
        assert_eq!(hangman.user_guess, "_____");
    }

    #[test]
    fn get_game_start_test() {
        let mut hangman = HangMan::new();
        let input = b"10";
        let mut output = Vec::new();

        hangman.game_start_details(&input[..], &mut output);
        // let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(hangman.guesses_allowed, 10);
    }
}
