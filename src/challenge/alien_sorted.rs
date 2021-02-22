use std::cmp;
use std::collections;

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    if words.is_empty() {
        return true;
    }
    let char_indices = order.chars().enumerate().map(|e| (e.1, e.0)).collect();

    let mut words_iter = words.iter();
    let mut previous = words_iter.next().unwrap();

    for word in words_iter {
        if !is_before(previous, word, &char_indices) {
            return false;
        }
        previous = word;
    }

    true
}

fn is_before(first: &str, second: &str, char_indexes: &collections::HashMap<char, usize>) -> bool {
    let mut first_chars = first.chars();
    let mut second_chars = second.chars();
    loop {
        let first_char = first_chars.next();
        let second_char = second_chars.next();
        if let Some(first_char) = first_char {
            if let Some(second_char) = second_char {
                let first_index = char_indexes.get(&first_char).unwrap_or(&0);
                let second_index = char_indexes.get(&second_char).unwrap_or(&0);
                match first_index.cmp(&second_index) {
                    cmp::Ordering::Less => return true,
                    cmp::Ordering::Greater => return false,
                    cmp::Ordering::Equal => (),
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();

        // WHEN
        let res = is_alien_sorted(words, order);

        // THEN
        assert_eq!(res, true);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();

        // WHEN
        let res = is_alien_sorted(words, order);

        // THEN
        assert_eq!(res, false);
    }

    #[test]
    fn it_should_validate_example3() {
        // GIVEN
        let words = vec!["apple".to_string(), "app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();

        // WHEN
        let res = is_alien_sorted(words, order);

        // THEN
        assert_eq!(res, false);
    }
}
