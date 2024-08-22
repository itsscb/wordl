use serde::{Deserialize, Serialize};

use crate::CharStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Game {
    word: String,
    submitted_words: Vec<Vec<CharStatus<String>>>,
    result: GameResult,
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
