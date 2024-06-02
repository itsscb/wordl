pub mod pages;
pub mod router;
pub mod storage;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
enum CharStatus<T> {
    NotContained(T),
    Contained(T),
    Match(T),
}

fn compare_strings(s1: &str, s2: &str) -> Vec<CharStatus<String>> {
    if s1.len() != s2.len() {
        panic!("Strings must have the same length");
    }

    let mut result = Vec::with_capacity(s1.len());
    let s1_chars: HashSet<char> = s1.chars().collect();

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            result.push(CharStatus::Match(c2.to_string()));
        } else if s1_chars.contains(&c2) {
            result.push(CharStatus::Contained(c2.to_string()));
        } else {
            result.push(CharStatus::NotContained(c2.to_string()));
        }
    }

    result
}
