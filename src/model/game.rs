use super::charstatus::{compare_strings, CharStatus};
use serde::{Deserialize, Serialize};

type Tries = usize;

const MAX_TRIES: Tries = 5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub word: Option<String>,
    pub submitted_words: Vec<Vec<CharStatus<String>>>,
    tries: usize,
    status: Status,
}

impl Game {
    #[must_use]
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
            let res = compare_strings(word, &answer.join(""));
            self.submitted_words.push(res);
            self.tries += 1;
            self.status = self.current_status();
        }
    }

    #[must_use]
    pub fn current_status(&self) -> Status {
        self.word.as_ref().map_or(Status::New, |_| {
            let word_count = self.submitted_words.len();
            match self.tries {
                0 => Status::New,
                1..MAX_TRIES => self
                    .submitted_words
                    .last()
                    .map_or(Status::InProgress, |words| {
                        if words.iter().all(|v| matches!(v, CharStatus::Match(_))) {
                            Status::Win(word_count)
                        } else {
                            Status::InProgress
                        }
                    }),
                _ => self
                    .submitted_words
                    .last()
                    .map_or(Status::Lose(word_count), |words| {
                        if words.iter().all(|v| matches!(v, CharStatus::Match(_))) {
                            Status::Win(word_count)
                        } else {
                            Status::Lose(word_count)
                        }
                    }),
            }
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum Status {
    New,
    Win(Tries),
    Lose(Tries),
    InProgress,
}
