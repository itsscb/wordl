// use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::CharStatus;

const MAX_TRIES: usize = 5;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Games(Vec<Game>);

// impl Games {
//     pub const fn new() -> Self {
//         Self(Vec::new())
//     }

//     pub fn new_game(&mut self, word: String) {
//         let game = Game::new();
//         self.0.push(game);
//     }

//     pub fn current_game(&self) -> Option<&Game> {
//         self.0.last()
//     }
// }

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
    // pub fn to_json(&self) -> String {
    //     serde_json::to_string_pretty(self).map_or(String::new(), |w| w)
    // }
    // pub fn get_word(&self) -> String {
    //     let mut rng = rand::thread_rng();
    //     self.words
    //         .choose(&mut rng)
    //         .map_or_else(String::new, |w| (*w).to_string())
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub word: Option<String>,
    pub submitted_words: Vec<Vec<CharStatus<String>>>,
    tries: usize,
    status: Status,
}

impl Game {
    pub const fn new() -> Self {
        Self {
            word: None,
            tries: 0,
            submitted_words: Vec::new(),
            status: Status::New,
        }
    }

    pub fn start(&mut self, word: String) {
        if self.word.is_none() && self.status == Status::New {
            self.status = Status::InProgress;
            self.word = Some(word);
        }
    }

    pub fn submit_answer(&mut self, answer: &[String]) {
        if let Some(ref word) = self.word {
            let res = crate::compare_strings(word, &answer.join(""));
            self.submitted_words.push(res);
            self.tries += 1;
            self.status = self.current_status();
        }
    }

    pub fn current_status(&self) -> Status {
        self.word.as_ref().map_or(Status::New, |_| {
            let word_count = self.submitted_words.len();
            if self.tries == 0 {
                Status::New
            } else if self.tries < MAX_TRIES {
                if self
                    .submitted_words
                    .last()
                    .unwrap()
                    .iter()
                    .all(|v| matches!(v, CharStatus::Match(_)))
                {
                    Status::Win(word_count)
                } else {
                    Status::InProgress
                }
            } else if self
                .submitted_words
                .last()
                .unwrap()
                .iter()
                .all(|v| matches!(v, CharStatus::Match(_)))
            {
                Status::Win(word_count)
            } else {
                Status::Lose(word_count)
            }
        })
    }
}

type Tries = usize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum Status {
    New,
    Win(Tries),
    Lose(Tries),
    InProgress,
}
