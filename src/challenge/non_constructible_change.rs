pub fn non_constructible_change(coins: &mut Vec<usize>) -> usize {
    if coins.is_empty() {
        return 1;
    }
    // sort the coins
    coins.sort_unstable();

    let mut coins_iter = coins.iter();
    let mut total = *coins_iter.next().unwrap();
    for coin in coins_iter {
        if *coin > total + 1 {
            break;
        }
        total += *coin;
    }

    total + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_simple_example1() {
        // GIVEN
        let mut coins = vec![1, 2, 5];

        // WHEN
        let res = non_constructible_change(&mut coins);

        // THEN
        assert_eq!(res, 4);
    }

    #[test]
    fn it_should_simple_example2() {
        // GIVEN
        let mut coins = vec![1, 1, 1, 5];

        // WHEN
        let res = non_constructible_change(&mut coins);

        // THEN
        assert_eq!(res, 4);
    }

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let mut coins = vec![5, 7, 1, 1, 2, 3, 22];

        // WHEN
        let res = non_constructible_change(&mut coins);

        // THEN
        assert_eq!(res, 20);
    }
}
