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

    pub fn new_lefty(value: T, left: BinaryTreeNode<T>) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value,
            left: Some(Box::new(left)),
            right: None,
        }
    }

    pub fn new_righty(value: T, right: BinaryTreeNode<T>) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value,
            left: None,
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

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&BinaryTreeNode<T>> {
        self.left.as_deref()
    }

    pub fn right(&self) -> Option<&BinaryTreeNode<T>> {
        self.right.as_deref()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
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

    #[test]
    fn it_should_return_true_if_node_is_leaf() {
        // GIVEN
        let root = BinaryTreeNode::new_leaf(123);

        // WHEN
        let leaf = root.is_leaf();

        // THEN
        assert_eq!(leaf, true);
    }

    #[test]
    fn it_should_return_false_if_node_is_leaf() {
        // GIVEN
        let root = BinaryTreeNode::new_lefty(123, BinaryTreeNode::new_leaf(42));

        // WHEN
        let leaf = root.is_leaf();

        // THEN
        assert_eq!(leaf, false);
    }
}
