pub trait Heap<E> {
    fn add(&mut self, e: E);
    fn pop(&mut self) -> E;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
