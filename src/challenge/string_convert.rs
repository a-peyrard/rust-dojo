use std::collections;

pub fn can_convert(first: String, second: String) -> bool {
    if first.len() != second.len() {
        return false;
    }
    if first == second {
        return true;
    }

    let mut mapping: collections::HashMap<char, char> = collections::HashMap::new();
    for (f, s) in first.chars().zip(second.chars()) {
        let mapped = mapping.get(&f);
        if let Some(c) = mapped {
            if c != &s {
                return false;
            }
        } else {
            mapping.insert(f, s);
        }
    }
    // if we are mapping all letters of the alphabet, we don't want them to be mapped in the
    // 26 letters, otherwise we can not do the transformation, we don't have a proper order
    mapping.len() != 26
        || mapping
            .values()
            .collect::<collections::HashSet<&char>>()
            .len()
            != 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let first = "aabcc".to_string();
        let second = "ccdee".to_string();

        // WHEN
        let convert = can_convert(first, second);

        // THEN
        assert_eq!(convert, true);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let first = "leetcode".to_string();
        let second = "codeleet".to_string();

        // WHEN
        let convert = can_convert(first, second);

        // THEN
        assert_eq!(convert, false);
    }

    #[test]
    fn it_should_validate_example3() {
        // GIVEN
        let first = "abcdefghijklmnopqrstuvwxyz".to_string();
        let second = "bcdefghijklmnopqrstuvwxyza".to_string();

        // WHEN
        let convert = can_convert(first, second);

        // THEN
        assert_eq!(convert, false);
    }

    #[test]
    fn it_should_validate_example4() {
        // GIVEN
        let first = "abcdefghijklmnopqrstuvwxyz".to_string();
        let second = "bcadefghijklmnopqrstuvwxzz".to_string();

        // WHEN
        let convert = can_convert(first, second);

        // THEN
        assert_eq!(convert, true);
    }

    #[test]
    fn it_should_validate_example5() {
        // GIVEN
        let first = "abcdefghijklmnopqrstuvwxyz".to_string();
        let second = "abcdefghijklmnopqrstuvwxyz".to_string();

        // WHEN
        let convert = can_convert(first, second);

        // THEN
        assert_eq!(convert, true);
    }
}
