pub trait Heap<E> {
    fn push(&mut self, e: E);
    fn pop(&mut self) -> E;
    fn push_pop(&mut self, e: E) -> E;
    fn peek(&self) -> &E;
    /// Return true if the specified element would be better than the actual
    /// top of the heap
    ///
    /// For example for a min heap:
    /// ```
    /// use rust_dojo::heap::Heap;
    /// use rust_dojo::heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::min_heap();
    /// heap.push(32);
    ///
    /// let better = heap.compare_to_top(&3);
    /// assert_eq!(better, true);
    /// ```
    fn compare_to_top(&self, elem: &E) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
