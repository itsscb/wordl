use super::charstatus::{compare_strings, CharStatus};
use serde::{Deserialize, Serialize};

type Tries = usize;

const MAX_TRIES: Tries = 5;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Game {
    pub word: Option<String>,
    pub submitted_words: Vec<Vec<CharStatus<String>>>,
    tries: Tries,
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

    pub fn start(&mut self, word: &str) {
        if self.word.is_none() && self.status == Status::New {
            self.status = Status::InProgress;
            self.word = Some(word.to_uppercase());
        }
    }

    pub fn submit_answer(&mut self, answer: &str) {
        if let Some(ref word) = self.word {
            let res = compare_strings(word, &answer.to_uppercase());
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

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::iter;
    #[test]
    fn new() {
        assert_eq!(
            Game {
                word: None,
                tries: 0,
                submitted_words: Vec::new(),
                status: Status::New,
            },
            Game::new()
        );
    }

    #[test]
    fn start() {
        let word: String = random_word(5);

        let want = Game {
            word: Some(word.to_uppercase()),
            submitted_words: Vec::new(),
            tries: 0,
            status: Status::InProgress,
        };

        let mut got = Game::new();
        got.start(&word);

        assert_eq!(got, want);
    }

    #[test]
    fn submit_answer() {
        let word = "hallo";
        let answer = "hello";

        let want = Game {
            word: Some(word.to_uppercase()),
            submitted_words: vec![compare_strings(
                &word.to_uppercase(),
                &answer.to_uppercase(),
            )],
            tries: 1,
            status: Status::InProgress,
        };

        let mut got = Game::new();
        got.start(word);
        got.submit_answer(answer);

        assert_eq!(got, want);
    }

    #[test]
    fn current_status() {
        let mut got = Game::new();

        assert_eq!(got.current_status(), Status::New);

        let word = "hallo";

        let want = Game {
            word: Some(word.to_uppercase()),
            submitted_words: Vec::new(),
            tries: 0,
            status: Status::InProgress,
        };
        got.start(word);

        assert_eq!(got, want);

        let answer = "hello";
        let want = Game {
            word: Some(word.to_uppercase()),
            submitted_words: vec![compare_strings(
                &word.to_uppercase(),
                &answer.to_uppercase(),
            )],
            tries: 1,
            status: Status::InProgress,
        };
        got.submit_answer(answer);
        assert_eq!(got, want);

        got.submit_answer(answer);
        got.submit_answer(answer);
        got.submit_answer(answer);
        got.submit_answer(answer);
        assert_eq!(got.current_status(), Status::Lose(5));

        let mut got = Game::new();
        got.start(word);
        got.submit_answer(word);
        assert_eq!(got.current_status(), Status::Win(1));

        let mut got = Game::new();
        got.start(word);
        got.submit_answer(answer);
        got.submit_answer(answer);
        got.submit_answer(word);
        assert_eq!(got.current_status(), Status::Win(3));
    }

    fn random_word(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let word: String = iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .filter(char::is_ascii_lowercase)
            .take(len)
            .collect();
        word
    }
}