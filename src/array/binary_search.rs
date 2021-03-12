use std::cmp;

pub fn binary_search<T: Ord>(arr: &[T], element: T) -> Option<usize> {
    binary_search_rec(arr, element, 0, (arr.len() as i32) - 1)
}

fn binary_search_rec<T: Ord>(arr: &[T], element: T, start: i32, end: i32) -> Option<usize> {
    if start > end {
        return None;
    }
    let mid = start + (end - start) / 2;
    match arr[mid as usize].cmp(&element) {
        cmp::Ordering::Less => binary_search_rec(arr, element, mid + 1, end),
        cmp::Ordering::Greater => binary_search_rec(arr, element, start, mid - 1),
        cmp::Ordering::Equal => Some(mid as usize),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_none_for_empty_slice() {
        // GIVEN
        let arr = [];

        // WHEN
        let res = binary_search(&arr, 12);

        // THEN
        assert_eq!(res, None);
    }

    #[test]
    fn it_should_return_none_for_slice_with_one_element_not_being_the_one() {
        // GIVEN
        let arr = [5];

        // WHEN
        let res = binary_search(&arr, -5);

        // THEN
        assert_eq!(res, None);
    }

    #[test]
    fn it_should_return_zero_for_slice_with_one_element_being_the_one() {
        // GIVEN
        let arr = [5];

        // WHEN
        let res = binary_search(&arr, 5);

        // THEN
        assert_eq!(res, Some(0));
    }

    #[test]
    fn it_should_return_index_if_element_inside_array() {
        // GIVEN
        let arr = [1, 3, 6, 12, 34];

        // WHEN
        let res = binary_search(&arr, 12);

        // THEN
        assert_eq!(res, Some(3));
    }

    #[test]
    fn it_should_return_index_if_element_inside_array_case2() {
        // GIVEN
        let arr = [-1, 0, 3, 5, 9, 12];

        // WHEN
        let res = binary_search(&arr, 9);

        // THEN
        assert_eq!(res, Some(4));
    }

    #[test]
    fn it_should_return_none_if_element_not_inside_array() {
        // GIVEN
        let arr = [1, 3, 6, 12, 34];

        // WHEN
        let res = binary_search(&arr, 5);

        // THEN
        assert_eq!(res, None);
    }

    #[test]
    fn it_should_return_none_if_element_not_inside_array_case2() {
        // GIVEN
        let arr = [-1, 0, 3, 5, 9, 12];

        // WHEN
        let res = binary_search(&arr, 2);

        // THEN
        assert_eq!(res, None);
    }
}
