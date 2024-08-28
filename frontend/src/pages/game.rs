use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::CharStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Game {
    word: String,
    submitted_words: Vec<Vec<CharStatus<String>>>,
    result: GameResult,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WordList {
    words: Vec<String>,
}

impl WordList {
    pub const fn new() -> Self {
        Self { words: Vec::new() }
    }
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).map_or(Self::new(), |w| w)
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).map_or(String::new(), |w| w)
    }
    pub fn get_word(&self) -> String {
        let mut rng = rand::thread_rng();
        self.words
            .choose(&mut rng)
            .map_or_else(String::new, |w| (*w).to_string())
    }
}

impl Game {
    #[allow(dead_code)]
    pub fn new(word: String, submitted_words: Vec<Vec<CharStatus<String>>>) -> Self {
        let result = submitted_words
            .clone()
            .into_iter()
            .last()
            .map_or(GameResult::Lose, |w| {
                if w.iter().all(|v| matches!(v, CharStatus::Match(_))) {
                    GameResult::Win
                } else {
                    GameResult::Lose
                }
            });

        Self {
            word,
            submitted_words,
            result,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum GameResult {
    Win,
    Lose,
}
