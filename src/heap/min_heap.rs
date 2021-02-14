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

    pub fn add(&mut self, e: E) -> &mut MinHeap<E> {
        self.heap.push(e);
        self
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn pop(&mut self) -> E {
        unimplemented!(); // fixme
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
}
