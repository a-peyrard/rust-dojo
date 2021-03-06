use crate::list::LinkedList;
use crate::tree::BinaryTreeNode;

pub fn binary_tree_to_linked_list(root: BinaryTreeNode<i32>) -> LinkedList<i32> {
    let mut list = LinkedList::new();
    binary_tree_to_linked_list_rec(&root, &mut list);

    list
}

pub fn binary_tree_to_linked_list_rec(node: &BinaryTreeNode<i32>, list: &mut LinkedList<i32>) {
    if let Some(right) = node.right() {
        binary_tree_to_linked_list_rec(right, list);
    }
    list.push(*node.value());
    if let Some(left) = node.left() {
        binary_tree_to_linked_list_rec(left, list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example_1() {
        // GIVEN
        //          100
        //        /     \
        //       50      200
        //      /  \     /  \
        //     25   75  125 350
        //      \   /
        //      30 60
        let root = BinaryTreeNode::new(
            100,
            BinaryTreeNode::new(
                50,
                BinaryTreeNode::new_righty(25, BinaryTreeNode::new_leaf(30)),
                BinaryTreeNode::new_lefty(75, BinaryTreeNode::new_leaf(60)),
            ),
            BinaryTreeNode::new(
                200,
                BinaryTreeNode::new_leaf(125),
                BinaryTreeNode::new_leaf(350),
            ),
        );

        // WHEN
        let list = binary_tree_to_linked_list(root);

        // THEN
        let vec = list.into_iter().collect::<Vec<i32>>();
        assert_eq!(vec, vec![25, 30, 50, 60, 75, 100, 125, 200, 350]);
    }
}
