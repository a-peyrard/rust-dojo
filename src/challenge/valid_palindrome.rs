pub fn is_palindrome(s: String) -> bool {
    // keep only alphanum chars
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    is_palindrome_rec(&chars[..])
}

fn is_palindrome_rec(chars: &[char]) -> bool {
    match chars {
        [] | [_] => true,
        [s, middle @ .., e] => s == e && is_palindrome_rec(middle),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let s = "A man, a plan, a canal: Panama".to_string();

        // WHEN
        let palindrome = is_palindrome(s);

        // THEN
        assert_eq!(palindrome, true);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let s = "race a car".to_string();

        // WHEN
        let palindrome = is_palindrome(s);

        // THEN
        assert_eq!(palindrome, false);
    }

    #[test]
    fn it_should_validate_anna() {
        // GIVEN
        let s = "Anna".to_string();

        // WHEN
        let palindrome = is_palindrome(s);

        // THEN
        assert_eq!(palindrome, true);
    }
}
