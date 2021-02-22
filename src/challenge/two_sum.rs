use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (idx, elem) in nums.iter().enumerate() {
        let looking_for = seen.get(&(target - elem));
        if let Some(looking_for_index) = looking_for {
            return vec![*looking_for_index, idx as i32];
        }
        seen.insert(elem, idx as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_example_1() {
        // GIVEN
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        // WHEN
        let res = two_sum(nums, target);

        // THEN
        assert_eq!(res, vec![0, 1])
    }

    #[test]
    fn it_should_solve_example_2() {
        // GIVEN
        let nums = vec![3, 2, 4];
        let target = 6;

        // WHEN
        let res = two_sum(nums, target);

        // THEN
        assert_eq!(res, vec![1, 2])
    }

    #[test]
    fn it_should_solve_example_3() {
        // GIVEN
        let nums = vec![3, 3];
        let target = 6;

        // WHEN
        let res = two_sum(nums, target);

        // THEN
        assert_eq!(res, vec![0, 1])
    }
}
