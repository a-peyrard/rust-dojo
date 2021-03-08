pub fn reverse_words_in_sentence(sentence: &str) -> String {
    let mut reversed_chars = sentence.chars().rev().collect::<Vec<char>>();

    let mut start: usize = 0;
    let len = reversed_chars.len();
    for index in 0..len {
        if reversed_chars[index] == ' ' {
            reverse_word(&mut reversed_chars, start, index - 1);
            start = index + 1;
        }
    }
    reverse_word(&mut reversed_chars, start, len - 1);

    reversed_chars.into_iter().collect()
}

fn reverse_word(sentence: &mut [char], mut start: usize, mut end: usize) {
    while start < end {
        sentence.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_not_reverse_single_word_sentence() {
        // GIVEN
        let s = "foo";

        // WHEN
        let res = reverse_words_in_sentence(s);

        // THEN
        assert_eq!(res, "foo");
    }

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let s = "Hello World";

        // WHEN
        let res = reverse_words_in_sentence(s);

        // THEN
        assert_eq!(res, "World Hello");
    }
}
