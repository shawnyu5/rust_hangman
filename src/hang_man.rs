use std::fs;

#[derive(Debug)]
struct HangMan {
    correct_word: String,
    user_guess: String,
}

impl HangMan {
    /// Create a new HangMan instance
    /// # returns
    /// a new hangman instance
    /// # Example:
    /// ```
    /// let hangman = HangMan::new();
    /// ```
    pub fn new() -> HangMan {
        HangMan {
            correct_word: String::new(),
            user_guess: String::new(),
        }
    }

    /// reads the currect word from answer file, and return the hangman struct
    /// # Arguments
    /// * `hangman` - the hangman struct to store the correct word
    ///
    /// # Returns
    /// * `hangman` - the hangman struct with the correct word added
    ///
    /// # Example
    /// ```
    /// let hangman = HangMan::new();
    /// hangman.get_answer()
    /// ```
    pub fn get_answer(&mut self) {
        let answer = fs::read_to_string("./src/answer.txt");
        self.correct_word = answer.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test if the answer is read correctly from file
    fn get_answer_test() {
        let mut hangman = HangMan {
            correct_word: String::new(),
            user_guess: String::new(),
        };

        let mut hangman = HangMan::new();
        hangman.get_answer();
        assert_eq!(hangman.correct_word, "apple\n");
    }
}
