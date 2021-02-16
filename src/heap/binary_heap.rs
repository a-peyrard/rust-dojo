use crate::heap::heap_trait::Heap;

pub struct BinaryHeap<E: Ord> {
    data: Vec<E>,
    cmp: fn(&E, &E) -> bool,
}

impl<E: Ord> Default for BinaryHeap<E> {
    fn default() -> Self {
        Self::min_heap()
    }
}

impl<E: Ord> BinaryHeap<E> {
    fn min_heap() -> Self {
        BinaryHeap {
            data: vec![],
            cmp: std::cmp::PartialOrd::lt,
        }
    }

    fn max_heap() -> Self {
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
            if (self.cmp)(&self.data[first], &self.data[index]) {
                target = first;
            }
        }
        if let Some(second) = children.1 {
            if (self.cmp)(&self.data[second], &self.data[index]) {
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

impl<E: Ord> Heap<E> for BinaryHeap<E> {
    /// Adds an element to the heap.
    fn add(&mut self, e: E) {
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

    fn len(&self) -> usize {
        self.data.len()
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
        heap.add(3);
        heap.add(-1);

        // THEN (noop assertion, we just want to ensure compilation and no panic)
        assert!(true);
    }

    #[test]
    fn it_should_get_heap_length() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.add(3);
        heap.add(-1);

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
        heap.add(3);
        heap.add(-1);

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
        heap.add(3);
        heap.add(-1);
        heap.add(5);
        heap.add(0);

        // WHEN
        let min = heap.pop();

        // THEN
        assert_eq!(min, -1);
    }

    #[test]
    fn it_should_pop_min_value_and_re_balance_heap() {
        // GIVEN
        let mut heap = BinaryHeap::min_heap();
        heap.add(3);
        heap.add(-1);
        heap.add(5);
        heap.add(0);
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
        heap.add(3);
        heap.add(-1);
        heap.add(5);
        heap.add(0);

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
        heap.add(3);
        heap.add(-1);
        heap.add(5);
        heap.add(0);

        // WHEN
        let mut exhausted = vec![];
        while !heap.is_empty() {
            exhausted.push(heap.pop());
        }

        // THEN
        assert_eq!(exhausted, vec![5, 3, 0, -1]);
    }
}
