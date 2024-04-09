use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const NUM_GUESSES: i32 = 5;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    word: String,
    letters_guessed: HashSet<u8>,
    parts_left: i32,
}

impl Hangman {
    pub fn new(seed: u64) -> Result<Hangman, std::io::Error> {
        let mut file = File::open("assets/words_alpha.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let words: Vec<&str> = contents.lines().collect();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let word_index = rng.gen_range(0..words.len());
        let rand_word = words[word_index];
        let letters_guessed: HashSet<u8> = HashSet::new();
        Ok(Hangman {
            word: rand_word.to_owned(),
            letters_guessed: letters_guessed,
            parts_left: NUM_GUESSES,
        })
    }

    pub fn show(&self) -> String {
        let bword = self.word.as_bytes();
        let mut display: Vec<char> = vec![];
        for i in 0..bword.len() {
            if self.letters_guessed.contains(&bword[i]) {
                display.push(bword[i] as char)
            } else {
                display.push('_');
            }
            if i != bword.len() - 1 {
                display.push(' ');
            }
        }
        display.iter().collect()
    }

    pub fn guess(&mut self, c: char) -> String {
        self.letters_guessed.insert(c as u8);
        if self.word.contains(c) {
            return self.show();
        } else {
            self.parts_left -= 1;
            if self.parts_left <= 0 {
                return format!("You lose. The word was {}", self.word);
            } else {
                return format!("Nope -- {}", self.show());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORD0: &str = "rebroadened";

    #[test]
    fn create_and_correct_guess0() {
        // asserting results kinda awkward
        let res = Hangman::new(0);
        assert!(!res.is_err(), "file io error from construction");
        let mut hm = res.unwrap();
        assert_eq!(
            hm,
            Hangman {
                word: WORD0.to_owned(),
                letters_guessed: HashSet::new(),
                parts_left: NUM_GUESSES
            }
        );
        assert_eq!(hm.guess('a'), "_ _ _ _ _ a _ _ _ _ _");
        assert_eq!(
            hm,
            Hangman {
                word: "rebroadened".to_owned(),
                letters_guessed: HashSet::from(['a' as u8]),
                parts_left: NUM_GUESSES
            }
        )
    }

    #[test]
    fn create_and_incorrect_guess0() {
        let res = Hangman::new(0);
        assert!(!res.is_err(), "file io error from construction");
        let mut hm = res.unwrap();
        assert_eq!(
            hm,
            Hangman {
                word: WORD0.to_owned(),
                letters_guessed: HashSet::new(),
                parts_left: NUM_GUESSES
            }
        );
        assert_eq!(hm.guess('z'), "Nope -- _ _ _ _ _ _ _ _ _ _ _");
        assert_eq!(
            hm,
            Hangman {
                word: "rebroadened".to_owned(),
                letters_guessed: HashSet::from(['z' as u8]),
                parts_left: NUM_GUESSES - 1
            }
        )
    }

    #[test]
    fn guess_all_right() {
        let res = Hangman::new(0);
        assert!(!res.is_err(), "file io error from construction");
        let mut hm = res.unwrap();
        assert_eq!(
            hm,
            Hangman {
                word: WORD0.to_owned(),
                letters_guessed: HashSet::new(),
                parts_left: NUM_GUESSES
            }
        );
        assert_eq!(hm.guess('r'), "r _ _ r _ _ _ _ _ _ _");
        assert_eq!(hm.guess('e'), "r e _ r _ _ _ e _ e _");
        assert_eq!(hm.guess('b'), "r e b r _ _ _ e _ e _");
        assert_eq!(hm.guess('o'), "r e b r o _ _ e _ e _");
        assert_eq!(hm.guess('a'), "r e b r o a _ e _ e _");
        assert_eq!(hm.guess('d'), "r e b r o a d e _ e d");
        assert_eq!(hm.guess('n'), "r e b r o a d e n e d");
        assert_eq!(hm.show(), "r e b r o a d e n e d")
    }

    #[test]
    fn lose() {
        let res = Hangman::new(0);
        assert!(!res.is_err(), "file io error from construction");
        let mut hm = res.unwrap();
        assert_eq!(
            hm,
            Hangman {
                word: WORD0.to_owned(),
                letters_guessed: HashSet::new(),
                parts_left: NUM_GUESSES
            }
        );
        assert_eq!(hm.guess('z'), "Nope -- _ _ _ _ _ _ _ _ _ _ _");
        assert_eq!(hm.guess('r'), "r _ _ r _ _ _ _ _ _ _");
        assert_eq!(hm.guess('y'), "Nope -- r _ _ r _ _ _ _ _ _ _");
        assert_eq!(hm.guess('e'), "r e _ r _ _ _ e _ e _");
        assert_eq!(hm.guess('x'), "Nope -- r e _ r _ _ _ e _ e _");
        assert_eq!(hm.guess('b'), "r e b r _ _ _ e _ e _");
        assert_eq!(hm.guess('w'), "Nope -- r e b r _ _ _ e _ e _");
        assert_eq!(hm.guess('o'), "r e b r o _ _ e _ e _");
        assert_eq!(hm.guess('v'), "You lose. The word was rebroadened");
    }
}
