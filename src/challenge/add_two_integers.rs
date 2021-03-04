use crate::list::LinkedList;

pub fn add_two_integers(i1: LinkedList<i32>, i2: LinkedList<i32>) -> LinkedList<i32> {
    let mut res = LinkedList::new();

    let mut i1_node = i1.peek_node();
    let mut i2_node = i2.peek_node();
    let mut reminder = 0;
    while i1_node.is_some() || i2_node.is_some() {
        let number = *i1_node.map(|n| n.element()).unwrap_or(&0)
            + *i2_node.map(|n| n.element()).unwrap_or(&0)
            + reminder;
        res.push(number % 10);
        reminder = number / 10;

        i1_node = i1_node.and_then(|node| node.next());
        i2_node = i2_node.and_then(|node| node.next());
    }
    if reminder > 0 {
        res.push(reminder)
    }

    // reverse the list, because we pushed the digits to the front
    res.reverse();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let mut i1 = LinkedList::new();
        i1.push(9);
        i1.push(9);
        i1.push(0);
        i1.push(1);

        let mut i2 = LinkedList::new();
        i2.push(2);
        i2.push(3);
        i2.push(7);

        // WHEN
        let res = add_two_integers(i1, i2);

        // THEN
        let res_vec: Vec<i32> = res.into_iter().collect();
        assert_eq!(res_vec, vec![8, 3, 1, 0, 1]);
    }
}
