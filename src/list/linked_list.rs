/*
   Simple linked list implementation.

   Got some help from the amazing book: https://rust-unofficial.github.io/too-many-lists
*/

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
        Node { element, next }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node::new(element, self.head.take()));

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

pub struct LinkedListIntoIter<T>(LinkedList<T>);

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIter<T>;

    fn into_iter(self) -> LinkedListIntoIter<T> {
        LinkedListIntoIter(self)
    }
}

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_do_push_pop_operations() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn it_should_peek_a_value_from_a_list() {
        // GIVEN
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // WHEN
        let elem = list.peek();

        // THEN
        assert_eq!(elem, Some(&3));
    }

    #[test]
    fn it_should_peek_none_if_list_is_empty() {
        // GIVEN
        let list = LinkedList::<i32>::new();

        // WHEN
        let elem = list.peek();

        // THEN
        assert_eq!(elem, None);
    }

    #[test]
    fn it_should_peek_a_mutable_value_from_a_list() {
        // GIVEN
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // WHEN
        if let Some(value) = list.peek_mut() {
            *value = 42
        }

        // THEN
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn it_should_iterate_with_into_iter() {
        // GIVEN
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // WHEN
        let mut iter = list.into_iter();

        // THEN
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn it_should_iterate_with_iter() {
        // GIVEN
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // WHEN
        let mut iter = list.iter();

        // THEN
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
