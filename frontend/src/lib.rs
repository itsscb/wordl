pub mod pages;
pub mod router;
pub mod storage;

mod input;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum CharStatus<T> {
    NotContained(T),
    Contained(T),
    Match(T),
    Unknown,
}

fn compare_strings(s1: &str, s2: &str) -> Vec<CharStatus<String>> {
    let mut result: Vec<CharStatus<String>> = Vec::with_capacity(s1.len());
    result.resize_with(s1.len(), || CharStatus::Unknown);

    let mut s1_char_count: HashMap<char, usize> = HashMap::new();
    let mut s2_char_count: HashMap<char, usize> = HashMap::new();

    for c in s1.chars() {
        *s1_char_count.entry(c).or_insert(0) += 1;
    }

    for ((c1, c2), res) in s1.chars().zip(s2.chars()).zip(result.iter_mut()) {
        if c1 == c2 {
            *res = CharStatus::Match(c2.to_string());
            *s2_char_count.entry(c2).or_insert(0) += 1;
        } else {
            *res = CharStatus::Unknown;
        }
    }

    for (res, c2) in result.iter_mut().zip(s2.chars()) {
        match res {
            CharStatus::Unknown => {
                let c1_count = s1_char_count.get(&c2).unwrap_or(&0);
                let c2_count = s2_char_count.get(&c2).unwrap_or(&0);

                if *c1_count > 0 && c1_count > c2_count {
                    *res = CharStatus::Contained(c2.to_string());
                    *s2_char_count.entry(c2).or_insert(0) += 1;
                } else {
                    *res = CharStatus::NotContained(c2.to_string());
                }
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_compare_strings() {
        let source = "HALLO";

        let want = vec![
            CharStatus::NotContained("0".to_owned()),
            CharStatus::NotContained("0".to_owned()),
            CharStatus::NotContained("0".to_owned()),
            CharStatus::NotContained("0".to_owned()),
            CharStatus::NotContained("0".to_owned()),
        ];
        let input = "00000";

        let got = compare_strings(source, input);
        assert_eq!(want, got);
        let source = "HALLO";

        let want = vec![
            CharStatus::NotContained("L".to_owned()),
            CharStatus::NotContained("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::NotContained("L".to_owned()),
        ];
        let input = "LLLLL";

        let got = compare_strings(source, input);
        assert_eq!(want, got);

        let want = vec![
            CharStatus::Match("H".to_owned()),
            CharStatus::Match("A".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Match("O".to_owned()),
        ];
        let input = "HALLO";

        let got = compare_strings(source, input);
        assert_eq!(want, got);

        let want = vec![
            CharStatus::Match("H".to_owned()),
            CharStatus::NotContained("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Match("O".to_owned()),
        ];
        let input = "HLLLO";

        let got = compare_strings(source, input);
        assert_eq!(want, got);

        let want = vec![
            CharStatus::Match("H".to_owned()),
            CharStatus::Contained("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::NotContained("I".to_owned()),
            CharStatus::NotContained("L".to_owned()),
        ];
        let input = "HLLIL";

        let got = compare_strings(source, input);
        assert_eq!(want, got);

        let want = vec![
            CharStatus::Contained("L".to_owned()),
            CharStatus::NotContained("L".to_owned()),
            CharStatus::Match("L".to_owned()),
            CharStatus::Contained("A".to_owned()),
            CharStatus::Match("O".to_owned()),
        ];
        let input = "LLLAO";

        let got = compare_strings(source, input);
        assert_eq!(want, got);
    }
}
