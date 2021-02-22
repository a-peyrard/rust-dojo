use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy = i32::MAX;
    for price in prices {
        buy = cmp::min(buy, price);
        profit = cmp::max(profit, price - buy);
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let prices = vec![7, 1, 5, 3, 6, 4];

        // WHEN
        let profit = max_profit(prices);

        // THEN
        assert_eq!(profit, 5);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let prices = vec![7, 6, 4, 3, 1];

        // WHEN
        let profit = max_profit(prices);

        // THEN
        assert_eq!(profit, 0);
    }
}
