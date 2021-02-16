use crate::heap::{Heap, BinaryHeap};

pub struct KHeap<E: 'static + Ord + Clone> {
    heap: Box<dyn Heap<E>>, // &'a mut dyn Heap<E>,
    size: usize,
}

impl<E: Ord + Clone> KHeap<E> {
    /// Creates a new KHeap which will retain the k minimum elements.
    pub fn min_with_size(s: usize) -> Self {
        KHeap {
            heap: Box::new(BinaryHeap::max_heap()),
            size: s,
        }
    }

    /// Creates a new KHeap which will retain the k maximum elements.
    pub fn max_with_size(s: usize) -> Self {
        KHeap {
            heap: Box::new(BinaryHeap::min_heap()),
            size: s,
        }
    }
}

impl<E: Ord + Clone> Heap<E> for KHeap<E> {
    fn push(&mut self, e: E) {
        if self.size > self.heap.len() {
            self.heap.push(e);
        } else {
            self.heap.push_pop(e);
        }
    }

    fn pop(&mut self) -> E {
        self.heap.pop()
    }

    fn push_pop(&mut self, e: E) -> E {
        self.heap.push_pop(e)
    }

    fn peek(&self) -> &E {
        self.heap.peek()
    }

    fn compare_to_top(&self, elem: &E) -> bool {
        self.heap.compare_to_top(elem)
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_push_to_min_heap_if_size_not_reached() {
        // GIVEN
        let mut k_heap = KHeap::min_with_size(3);

        // WHEN
        k_heap.push(3);
        let head = k_heap.peek();

        // THEN
        assert_eq!(*head, 3);
    }

    #[test]
    fn it_should_push_to_min_heap_only_if_element_is_smaller_than_max() {
        // GIVEN
        let mut k_heap = KHeap::min_with_size(3);
        k_heap.push(3);
        k_heap.push(0);
        k_heap.push(1);

        // WHEN
        k_heap.push(2);
        let head = k_heap.peek();
        let len = k_heap.len();

        // THEN
        assert_eq!(*head, 2);
        assert_eq!(len, 3);
    }

    #[test]
    fn it_should_push_to_min_heap_only_if_element_is_smaller_than_max_and_not_the_new_max() {
        // GIVEN
        let mut k_heap = KHeap::min_with_size(3);
        k_heap.push(3);
        k_heap.push(-1);
        k_heap.push(1);

        // WHEN
        k_heap.push(0);
        let head = k_heap.peek();
        let len = k_heap.len();

        // THEN
        assert_eq!(*head, 1);
        assert_eq!(len, 3);
    }

    #[test]
    fn it_should_not_push_to_min_heap_if_element_is_bigger_than_max() {
        // GIVEN
        let mut k_heap = KHeap::min_with_size(3);
        k_heap.push(3);
        k_heap.push(0);
        k_heap.push(1);

        // WHEN
        k_heap.push(18);
        let head = k_heap.peek();
        let len = k_heap.len();

        // THEN
        assert_eq!(*head, 3);
        assert_eq!(len, 3);
    }

    #[test]
    fn it_should_push_to_max_heap_only_if_element_is_bigger_than_min() {
        // GIVEN
        let mut k_heap = KHeap::max_with_size(3);
        k_heap.push(3);
        k_heap.push(0);
        k_heap.push(1);

        // WHEN
        k_heap.push(2);
        let head = k_heap.peek();
        let len = k_heap.len();

        // THEN
        assert_eq!(*head, 1);
        assert_eq!(len, 3);
    }
}