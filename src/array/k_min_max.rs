use crate::heap::{KHeap, Heap};

pub fn find_k_minimum<T: Ord>(arr: &[T], k: usize) -> Option<&T> {
    if arr.len() < k {
        return None;
    }

    let mut kheap = KHeap::min_with_size(k);
    for e in arr.iter() {
        kheap.push(e);
    }
    Some(kheap.peek())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_none_if_not_enough_elements() {
        // GIVEN
        let arr = [4, -1, 18, 0];

        // WHEN
        let res = find_k_minimum(&arr, 5);

        // THEN
        assert!(res.is_none());
    }

    #[test]
    fn it_should_return_1_minimum() {
        // GIVEN
        let arr = [4, -1, 18, 0];

        // WHEN
        let res = find_k_minimum(&arr, 1);

        // THEN
        assert_eq!(*res.unwrap(), -1);
    }

    #[test]
    fn it_should_return_2_minimum() {
        // GIVEN
        let arr = [4, -1, 18, 0];

        // WHEN
        let res = find_k_minimum(&arr, 2);

        // THEN
        assert_eq!(*res.unwrap(), 0);
    }

    #[test]
    fn it_should_return_3_minimum() {
        // GIVEN
        let arr = [4, -1, 18, 0];

        // WHEN
        let res = find_k_minimum(&arr, 3);

        // THEN
        assert_eq!(*res.unwrap(), 4);
    }

    #[test]
    fn it_should_return_4_minimum() {
        // GIVEN
        let arr = [4, -1, 18, 0];

        // WHEN
        let res = find_k_minimum(&arr, 4);

        // THEN
        assert_eq!(*res.unwrap(), 18);
    }
}