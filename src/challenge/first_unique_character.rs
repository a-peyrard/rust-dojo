pub fn first_uniq_char(s: String) -> i32 {
    let a_codepoint = b'a';
    let mut characters: [i32; 26] = [-2; 26];
    for (index, char) in s.chars().enumerate() {
        let char_index = (char as u8 - a_codepoint) as usize;
        characters[char_index] = match characters[char_index] {
            -2 => index as i32,
            _ => -1,
        }
    }

    *characters.iter().filter(|i| **i >= 0).min().unwrap_or(&-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let s = "leetcode".to_string();

        // WHEN
        let res = first_uniq_char(s);

        // THEN
        assert_eq!(res, 0);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let s = "loveleetcode".to_string();

        // WHEN
        let res = first_uniq_char(s);

        // THEN
        assert_eq!(res, 2);
    }

    #[test]
    fn it_should_return_minus_one_if_all_character_are_repeating() {
        // GIVEN
        let s = "aaaa".to_string();

        // WHEN
        let res = first_uniq_char(s);

        // THEN
        assert_eq!(res, -1);
    }
}
