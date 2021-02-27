pub fn frequency_sort(s: String) -> String {
    let mut occurrences = [0; 255];
    for character in s.chars() {
        occurrences[get_char_index(character)] += 1;
    }

    let mut chars_vec = s.chars().collect::<Vec<char>>();
    chars_vec.sort_by(|a, b| {
        let a_i = get_char_index(*a);
        let b_i = get_char_index(*b);
        let a_o = occurrences[a_i];
        let b_o = occurrences[b_i];

        if a_o == b_o {
            b_i.cmp(&a_i)
        } else {
            b_o.cmp(&a_o)
        }
    });

    chars_vec.into_iter().collect::<String>()
}

fn get_char_index(character: char) -> usize {
    character as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let s = "tree".to_string();

        // WHEN
        let res = frequency_sort(s);

        // THEN
        assert!(["eert", "eetr"].contains(&&res[..]));
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let s = "cccaaa".to_string();

        // WHEN
        let res = frequency_sort(s);

        // THEN
        assert!(["aaaccc", "cccaaa"].contains(&&res[..]));
    }

    #[test]
    fn it_should_validate_example3() {
        // GIVEN
        let s = "Aabb".to_string();

        // WHEN
        let res = frequency_sort(s);

        // THEN
        assert!(["bbAa", "bbaA"].contains(&&res[..]));
    }

    #[test]
    fn it_should_validate_example4() {
        // GIVEN
        let s = "loveleetcode".to_string();

        // WHEN
        let res = frequency_sort(s);

        // THEN
        assert!(["eeeeoollvtdc"].contains(&&res[..]));
    }

    #[test]
    fn it_should_not_panic_with_numbers() {
        // GIVEN
        let s = "2a554442f544asfasssffffasss".to_string();

        // WHEN
        frequency_sort(s);
    }
}
