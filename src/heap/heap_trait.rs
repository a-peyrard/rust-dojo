pub trait Heap<E> {
    fn min_heap() -> Self;
    fn max_heap() -> Self;

    fn add(&mut self, e: E) -> &mut Self;
    fn pop(&mut self) -> E;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
