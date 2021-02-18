use crate::heap::heap_trait::Heap;

pub struct BinaryHeap<E: Ord + Clone> {
    data: Vec<E>,
    cmp: fn(&E, &E) -> bool,
}

impl<E: Ord + Clone> Default for BinaryHeap<E> {
    fn default() -> Self {
        Self::min_heap()
    }
}

impl<E: Ord + Clone> BinaryHeap<E> {
    pub fn min_heap() -> Self {
        BinaryHeap {
            data: vec![],
            cmp: std::cmp::PartialOrd::lt,
        }
    }

    pub fn max_heap() -> Self {
        BinaryHeap {
            data: vec![],
            cmp: std::cmp::PartialOrd::gt,
        }
    }

    fn up_heap(&mut self, index: usize) {
        let parent_index = self.parent_index(index);
        if let Some(parent_index) = parent_index {
            if (self.cmp)(&self.data[index], &self.data[parent_index]) {
                self.data.swap(index, parent_index);
                self.up_heap(parent_index);
            }
        }
    }

    fn down_heap(&mut self, index: usize) {
        let children = self.children_indexes(index);
        let mut target = index;
        if let Some(first) = children.0 {
            if (self.cmp)(&self.data[first], &self.data[target]) {
                target = first;
            }
        }
        if let Some(second) = children.1 {
            if (self.cmp)(&self.data[second], &self.data[target]) {
                target = second;
            }
        }
        if target != index {
            self.data.swap(target, index);
            self.down_heap(target);
        }
    }

    fn parent_index(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            _ => Some((index - 1) / 2)
        }
    }

    fn children_indexes(&self, index: usize) -> (Option<usize>, Option<usize>) {
        (
            self.to_valid_index(index * 2 + 1),
            self.to_valid_index(index * 2 + 2),
        )
    }

    fn to_valid_index(&self, index: usize) -> Option<usize> {
        if index >= self.data.len() {
            None
        } else {
            Some(index)
        }
    }
}

impl<E: Ord + Clone> Heap<E> for BinaryHeap<E> {
    /// Adds an element to the heap.
    fn push(&mut self, e: E) {
        /*
            - Add the element to the bottom level of the heap at the leftmost open space.
            - Compare the added element with its parent; if they are in the correct order, stop.
            - If not, swap the element with its parent and return to the previous step.
         */
        self.data.push(e);
        self.up_heap(self.data.len() - 1);
    }

    fn pop(&mut self) -> E {
        /*
            Replace the root of the heap with the last element on the last level.
            Compare the new root with its children; if they are in the correct order, stop.
            If not, swap the element with one of its children and return to the previous step.
            (Swap with its smaller child in a min-heap and its larger child in a max-heap.)
         */
        let head = self.data.swap_remove(0);
        self.down_heap(0);
        head
    }

    fn push_pop(&mut self, e: E) -> E {
        /*
            Compare whether the item we're pushing or the peeked top of the heap is greater
            (assuming a max heap)
            If the root of the heap is greater:
                Replace the root with the new item
                Down-heapify starting from the root
            Else, return the item we're pushing
         */
        let better = self.compare_to_top(&e);
        if better {
            e
        } else {
            let res = std::mem::replace(&mut self.data[0], e);
            self.down_heap(0);
            res
        }
    }

    fn peek(&self) -> &E {
        &self.data[0]
    }

    fn compare_to_top(&self, elem: &E) -> bool {
        (self.cmp)(elem, self.peek())
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<E: 'static + Ord + Clone> IntoIterator for &BinaryHeap<E> {
    type Item = E;
    type IntoIter = BinaryHeapIntoIterator<E>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryHeapIntoIterator {
            heap: BinaryHeap {
                data: self.data.to_vec(),
                cmp: self.cmp,
            },
        }
    }
}

pub struct BinaryHeapIntoIterator<E: 'static + Ord + Clone> {
    heap: BinaryHeap<E>,
}

impl<E: Ord + Clone> Iterator for BinaryHeapIntoIterator<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap.len() {
            0 => None,
            _ => Some(self.heap.pop())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_allow_to_push_element_in_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();

        // WHEN
        heap.push(3);
        heap.push(-1);

        // THEN (noop assertion, we just want to ensure compilation and no panic)
    }

    #[test]
    fn it_should_get_heap_length() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);

        // WHEN
        let length = heap.len();

        // THEN
        assert_eq!(length, 2);
    }

    #[test]
    fn it_should_check_emptiness_for_empty_heap() {
        // GIVEN
        let heap: BinaryHeap<i32> = BinaryHeap::min_heap();

        // WHEN
        let empty = heap.is_empty();

        // THEN
        assert!(empty);
    }

    #[test]
    fn it_should_check_emptiness_for_non_empty_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);

        // WHEN
        let empty = heap.is_empty();

        // THEN
        assert!(!empty);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_if_pop_empty_heap() {
        // GIVEN
        let mut heap: BinaryHeap<i32> = BinaryHeap::min_heap();

        // WHEN ... should panic 😱
        heap.pop();
    }

    #[test]
    fn it_should_pop_min_value() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let min = heap.pop();

        // THEN
        assert_eq!(min, -1);
    }

    #[test]
    fn it_should_pop_min_value_and_re_balance_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);
        heap.pop();

        // WHEN
        let min = heap.pop();

        // THEN
        assert_eq!(min, 0);
    }

    #[test]
    fn it_should_exhaust_min_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let mut exhausted = vec![];
        while !heap.is_empty() {
            exhausted.push(heap.pop());
        }

        // THEN
        assert_eq!(exhausted, vec![-1, 0, 3, 5]);
    }

    #[test]
    fn it_should_exhaust_max_heap() {
        // GIVEN
        let mut heap = BinaryHeap::max_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let mut exhausted = vec![];
        while !heap.is_empty() {
            exhausted.push(heap.pop());
        }

        // THEN
        assert_eq!(exhausted, vec![5, 3, 0, -1]);
    }

    #[test]
    fn it_should_peek_min_value() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let min = heap.peek();
        let length = heap.len();

        // THEN
        assert_eq!(*min, -1);
        assert_eq!(length, 4);
    }

    #[test]
    fn it_should_compare_with_top_for_min_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let better = heap.compare_to_top(&-3);
        let not_better = heap.compare_to_top(&0);

        // THEN
        assert_eq!(better, true);
        assert_eq!(not_better, false);
    }

    #[test]
    fn it_should_compare_with_top_for_max_heap() {
        // GIVEN
        let mut heap = BinaryHeap::max_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let better = heap.compare_to_top(&6);
        let not_better = heap.compare_to_top(&4);

        // THEN
        assert_eq!(better, true);
        assert_eq!(not_better, false);
    }

    #[test]
    fn it_should_push_pop_for_min_heap_when_elem_is_smaller() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let value = heap.push_pop(-3);

        // THEN
        assert_eq!(value, -3);
    }

    #[test]
    fn it_should_push_pop_for_min_heap_when_elem_is_bigger() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let value = heap.push_pop(1);
        let new_head = heap.peek();

        // THEN
        assert_eq!(value, -1);
        assert_eq!(*new_head, 0);
    }

    #[test]
    fn it_should_push_pop_and_promote_minimum_children() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.push(0);
        heap.push(4);
        heap.push(18);

        // WHEN
        heap.push_pop(22);
        let head = heap.peek();

        // THEN
        assert_eq!(*head, 4);
    }

    #[test]
    fn it_should_iterate_over_values() {
        // GIVEN
        let mut heap: BinaryHeap<i32> = BinaryHeap::min_heap();
        heap.push(3);
        heap.push(-1);
        heap.push(5);
        heap.push(0);

        // WHEN
        let mut iterated = vec![];
        for e in &heap {
            iterated.push(e);
        }
        let len_of_original_heap = heap.len();

        // THEN
        assert_eq!(iterated, vec![-1, 0, 3, 5]);
        assert_eq!(len_of_original_heap, 4);
    }
}
