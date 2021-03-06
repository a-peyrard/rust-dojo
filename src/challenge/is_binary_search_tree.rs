use crate::tree::BinaryTreeNode;

pub fn is_binary_search_tree(root: &BinaryTreeNode<i32>) -> bool {
    is_binary_search_tree_rec(root, i32::MIN, i32::MAX)
}

pub fn is_binary_search_tree_rec(node: &BinaryTreeNode<i32>, min: i32, max: i32) -> bool {
    if *node.value() < min || *node.value() > max {
        return false;
    }
    if let Some(left) = node.left() {
        if !is_binary_search_tree_rec(left, min, *node.value()) {
            return false;
        }
    }
    if let Some(right) = node.right() {
        if !is_binary_search_tree_rec(right, *node.value(), max) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_leaf() {
        // GIVEN

        let root = BinaryTreeNode::new_leaf(25);

        // WHEN
        let binary_search_tree = is_binary_search_tree(&root);

        // THEN
        assert_eq!(binary_search_tree, true);
    }

    #[test]
    fn it_should_validate_minimal_tree() {
        // GIVEN

        let root = BinaryTreeNode::new(2, BinaryTreeNode::new_leaf(1), BinaryTreeNode::new_leaf(3));

        // WHEN
        let binary_search_tree = is_binary_search_tree(&root);

        // THEN
        assert_eq!(binary_search_tree, true);
    }

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        //          100
        //        /     \
        //       50     200
        //      /  \    /  \
        //     25  75  125 350
        let root = BinaryTreeNode::new(
            100,
            BinaryTreeNode::new(
                50,
                BinaryTreeNode::new_leaf(25),
                BinaryTreeNode::new_leaf(75),
            ),
            BinaryTreeNode::new(
                200,
                BinaryTreeNode::new_leaf(125),
                BinaryTreeNode::new_leaf(350),
            ),
        );

        // WHEN
        let binary_search_tree = is_binary_search_tree(&root);

        // THEN
        assert_eq!(binary_search_tree, true);
    }

    #[test]
    fn it_should_validate_example2() {
        // GIVEN
        //          100
        //        /     \
        //       50     200
        //      /  \    /  \
        //     25  75  90  350
        let root = BinaryTreeNode::new(
            100,
            BinaryTreeNode::new(
                50,
                BinaryTreeNode::new_leaf(25),
                BinaryTreeNode::new_leaf(75),
            ),
            BinaryTreeNode::new(
                200,
                BinaryTreeNode::new_leaf(90),
                BinaryTreeNode::new_leaf(350),
            ),
        );

        // WHEN
        let binary_search_tree = is_binary_search_tree(&root);

        // THEN
        assert_eq!(binary_search_tree, false);
    }
}
