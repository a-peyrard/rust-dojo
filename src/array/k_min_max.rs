use crate::heap::{Heap, KHeap};

pub fn find_k_minimum<T: Ord + Clone>(arr: &[T], k: usize) -> Option<&T> {
    find_k_min_max(arr, k, KHeap::min_with_size)
}

pub fn find_k_maximum<T: Ord + Clone>(arr: &[T], k: usize) -> Option<&T> {
    find_k_min_max(arr, k, KHeap::max_with_size)
}

fn find_k_min_max<'a, T: 'a + Ord + Clone>(
    arr: &'a [T],
    k: usize,
    k_heap_initializer: fn(usize) -> KHeap<'a, &'a T>,
) -> Option<&'a T> {
    if arr.len() < k {
        return None;
    }

    let mut kheap = (k_heap_initializer)(k);
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

    #[test]
    fn it_should_return_3_maximum() {
        // GIVEN
        let arr = [4, -1, 18, 0, 22, 42, 3];

        // WHEN
        let res = find_k_maximum(&arr, 3);

        // THEN
        assert_eq!(*res.unwrap(), 18);
    }
}
