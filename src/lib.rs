mod words;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    hints: Vec<(u8, Hint)>,
}

#[derive(Debug, Copy, Clone)]
pub enum Hint {
    Correct(usize),
    WrongSpot(usize),
    NotInWord,
}

impl Game {
    pub fn new() -> Self {
        Game { hints: Vec::new() }
    }

    pub fn best_guess(&self) -> &'static str {
        let possible_words: Vec<_> = words::WORDS
            .iter()
            .filter(|word| {
                let word = word.as_bytes();
                for (letter, hint) in self.hints.iter().copied() {
                    match hint {
                        Hint::Correct(idx) => {
                            if word[idx] != letter {
                                return false;
                            }
                        }
                        Hint::WrongSpot(idx) => {
                            if !word.contains(&letter) || word[idx] == letter {
                                return false;
                            }
                        }
                        Hint::NotInWord => {
                            if word.contains(&letter) {
                                return false;
                            }
                        }
                    }
                }
                true
            })
            .collect();
        let mut freq = HashMap::new();
        for word in &possible_words {
            for letter in unique_letters(word) {
                *freq.entry(letter).or_insert(0) += 1;
            }
        }
        let mut freq_dbg: Vec<_> = freq.iter().map(|(l, w)| (*l as char, w)).collect();
        freq_dbg.sort_by_key(|(_letter, count)| *count);
        freq_dbg.reverse();
        eprintln!("{:?}", freq_dbg);
        dbg!(word_score("soare", &freq));
        dbg!(word_score("later", &freq));
        eprintln!("{:} possible words", possible_words.len());
        possible_words
            .into_iter()
            .map(|word| (word, word_score(word, &freq)))
            .max_by_key(|(_word, score)| -> usize { *score })
            .map(|(word, _score)| word)
            .unwrap()
    }

    pub fn add_hint(&mut self, letter: u8, hint: Hint) {
        self.hints.push((letter, hint));
    }
}

fn word_score(word: &str, freq: &HashMap<u8, usize>) -> usize {
    unique_letters(word)
        .map(|letter| freq.get(&letter).copied().unwrap_or(0))
        .sum()
}

/// Iterate over the unique letters of a string.
/// The string must only contain 'a'..='z' ascii characters.
fn unique_letters(s: &str) -> impl Iterator<Item = u8> + '_ {
    let mut seen = 0u32;
    s.bytes().filter(move |&letter| {
        assert!((b'a'..=b'z').contains(&letter));
        let bit_num = letter - b'a';
        let not_yet_seen = seen & (1 << bit_num) == 0;
        seen |= 1 << bit_num;
        not_yet_seen
    })
}
