use std::collections;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (first, second) = if nums1.len() < nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let mut seen = collections::HashMap::<i32, i32>::new();
    for num in first {
        *seen.entry(num).or_insert(0) += 1
    }

    let mut res = Vec::<i32>::new();
    for num in second {
        if let Some(entry) = seen.get_mut(&num) {
            if *entry > 0 {
                res.push(num);
                *entry -= 1;
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
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];

        // WHEN
        let intersection = intersect(nums1, nums2);

        // THEN
        assert_eq!(intersection, vec![2, 2]);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        // WHEN
        let intersection = intersect(nums1, nums2);

        // THEN
        assert_eq!(intersection.len(), 2);
        assert_eq!(intersection.contains(&4), true);
        assert_eq!(intersection.contains(&9), true);
    }
}
