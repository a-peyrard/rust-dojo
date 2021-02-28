use std::collections;

pub fn frequency_sort(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let mut occurrences = collections::HashMap::<char, usize>::new();
    let mut max = 0;
    for character in s.chars() {
        let counter = occurrences.entry(character).or_insert(0);
        *counter += 1;
        if max < *counter {
            max = *counter;
        }
    }

    // create buckets
    let mut buckets: Vec<Vec<char>> = vec![vec![]; max];
    for (character, occurrence) in occurrences {
        buckets[occurrence - 1].push(character);
    }

    // re-insert the characters
    let mut res = String::new();
    let len = buckets.len();
    for (index, bucket) in buckets.iter().rev().enumerate() {
        let occurrence = len - index;
        for character in bucket {
            for _ in 0..occurrence {
                res.push(*character);
            }
        }
    }

    res
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
    fn it_should_not_panic_with_numbers() {
        // GIVEN
        let s = "2a554442f544asfasssffffasss".to_string();

        // WHEN
        frequency_sort(s);
    }
}
