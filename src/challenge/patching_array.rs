pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let len = nums.len();
    let mut total: i64 = 0;
    let mut index = 0;
    let mut patches = 0;
    while total < n as i64 {
        if index >= len || nums[index] as i64 > total + 1 {
            patches += 1;
            total = total + total + 1;
        } else {
            total += nums[index] as i64;
            index += 1
        }
    }

    patches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_example1() {
        // GIVEN
        let nums = vec![1, 3];
        let n = 6;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 1);
    }

    #[test]
    fn it_should_solve_example2() {
        // GIVEN
        let nums = vec![1, 5, 10];
        let n = 20;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 2);
    }

    #[test]
    fn it_should_solve_example3() {
        // GIVEN
        let nums = vec![1, 2, 2];
        let n = 5;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 0);
    }

    #[test]
    fn it_should_solve_example4() {
        // GIVEN
        let nums = vec![1, 2, 31, 33];
        let n = 2147483647;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 28);
    }

    #[test]
    fn it_should_solve_example5() {
        // GIVEN
        let nums = vec![1, 2, 3, 8];
        let n = 80;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 3);
    }

    #[test]
    fn it_should_solve_empty_array1() {
        // GIVEN
        let nums = vec![];
        let n = 1;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 1);
    }

    #[test]
    fn it_should_solve_empty_array2() {
        // GIVEN
        let nums = vec![];
        let n = 8;

        // WHEN
        let res = min_patches(nums, n);

        // THEN
        assert_eq!(res, 4);
    }
}
