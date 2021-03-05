#[allow(dead_code)]
pub struct BinaryTreeNode<T> {
    value: T,
    left: Option<Box<BinaryTreeNode<T>>>,
    right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(value: T, left: BinaryTreeNode<T>, right: BinaryTreeNode<T>) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn new_leaf(value: T) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_allow_to_easily_build_a_tree() {
        // GIVEN

        // WHEN
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

        // THEN
        assert_eq!(root.value, 100);
    }
}
