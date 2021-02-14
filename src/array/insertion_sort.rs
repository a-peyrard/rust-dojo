pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if arr[j] >= arr[j - 1] {
                break;
            }
            arr.swap(j, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sort_sorted_array() {
        // GIVEN
        let mut arr = [0, 1, 2, 3, 4, 5];

        // WHEN
        insertion_sort(&mut arr);

        // THEN
        assert_eq!(arr, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_should_sort_reverse_sorted_array() {
        // GIVEN
        let mut arr = [5, 4, 3, 2, 1, 0];

        // WHEN
        insertion_sort(&mut arr);

        // THEN
        assert_eq!(arr, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_should_sort_custom_array() {
        // GIVEN
        let mut arr = [4, -2, 3, 19, 1, -28];

        // WHEN
        insertion_sort(&mut arr);

        // THEN
        assert_eq!(arr, [-28, -2, 1, 3, 4, 19]);
    }

    #[test]
    fn it_should_sort_u8_custom_array() {
        // GIVEN
        let mut arr: [u8; 6] = [4, 2, 3, 19, 1, 28];

        // WHEN
        insertion_sort(&mut arr);

        // THEN
        assert_eq!(arr, [1, 2, 3, 4, 19, 28]);
    }
}
