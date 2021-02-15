#[derive(Debug)]
pub struct MinHeap<E: Ord> {
    heap: Vec<E>,
}

impl <E: Ord> Default for MinHeap<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Ord> MinHeap<E> {
    pub fn new() -> Self {
        MinHeap { heap: vec![] }
    }

    /// Adds an element to the heap.
    pub fn add(&mut self, e: E) -> &mut MinHeap<E> {
        /*
            - Add the element to the bottom level of the heap at the leftmost open space.
            - Compare the added element with its parent; if they are in the correct order, stop.
            - If not, swap the element with its parent and return to the previous step.
         */
        self.heap.push(e);
        self.up_heap(self.heap.len() - 1);
        self
    }

    fn up_heap(&mut self, index: usize) {
        let parent_index = self.parent_index(index);
        if let Some(parent_index) = parent_index {
            if self.heap[parent_index] > self.heap[index] {
                self.heap.swap(index, parent_index);
                self.up_heap(parent_index);
            }
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
        if index >= self.heap.len() {
            None
        } else {
            Some(index)
        }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&mut self) -> E {
        /*
            Replace the root of the heap with the last element on the last level.
            Compare the new root with its children; if they are in the correct order, stop.
            If not, swap the element with one of its children and return to the previous step.
            (Swap with its smaller child in a min-heap and its larger child in a max-heap.)
         */
        let head = self.heap.swap_remove(0);
        self.down_heap(0);
        head
    }

    fn down_heap(&mut self, index: usize) {
        let children = self.children_indexes(index);
        let mut min = index;
        if let Some(first) = children.0 {
            if self.heap[first] < self.heap[index] {
                min = first;
            }
        }
        if let Some(second) = children.1 {
            if self.heap[second] < self.heap[index] {
                min = second;
            }
        }
        if min != index {
            self.heap.swap(min, index);
            self.down_heap(min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_allow_to_push_element_in_heap() {
        // GIVEN
        let mut heap = MinHeap::new();

        // WHEN
        heap.add(3)
            .add(-1);

        // THEN (noop assertion, we just want to ensure compilation and no panic)
        assert_eq!(1, 1);
    }

    #[test]
    fn it_should_get_heap_length() {
        // GIVEN
        let mut heap = MinHeap::new();
        heap.add(3)
            .add(-1);

        // WHEN
        let length = heap.len();

        // THEN
        assert_eq!(length, 2);
    }

    #[test]
    fn it_should_check_emptiness_for_empty_heap() {
        // GIVEN
        let heap: MinHeap<i32> = MinHeap::new();

        // WHEN
        let empty = heap.is_empty();

        // THEN
        assert!(empty);
    }

    #[test]
    fn it_should_check_emptiness_for_non_empty_heap() {
        // GIVEN
        let mut heap = MinHeap::new();
        heap.add(3)
            .add(-1);

        // WHEN
        let empty = heap.is_empty();

        // THEN
        assert!(!empty);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_if_pop_empty_heap() {
        // GIVEN
        let mut heap: MinHeap<i32> = MinHeap::new();

        // WHEN ... should panic 😱
        heap.pop();
    }

    #[test]
    fn it_should_pop_min_value() {
        // GIVEN
        let mut heap = MinHeap::new();
        heap.add(3)
            .add(-1)
            .add(5)
            .add(0);

        // WHEN
        let min = heap.pop();

        // THEN
        assert_eq!(min, -1);
    }

    #[test]
    fn it_should_pop_min_value_and_re_balance_heap() {
        // GIVEN
        let mut heap = MinHeap::new();
        heap.add(3)
            .add(-1)
            .add(5)
            .add(0)
            .pop();

        // WHEN
        let min = heap.pop();

        // THEN
        assert_eq!(min, 0);
    }
}
