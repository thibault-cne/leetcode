use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn leaf_value_seq(root: Rc<RefCell<TreeNode>>) -> Vec<i32> {
        let mut leaf_value_seq = Vec::new();
        let mut queue = Vec::new();

        // Processing root1
        queue.push(root);
        while let Some(node) = queue.pop() {
            if node.borrow().is_leaf() {
                leaf_value_seq.push(node.borrow().val);
            } else {
                if let Some(left) = node.borrow().left.clone() {
                    queue.push(left);
                }

                if let Some(right) = node.borrow().right.clone() {
                    queue.push(right);
                }
            }
        }

        leaf_value_seq
    }
}

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (Some(root1), Some(root2)) => {
                TreeNode::leaf_value_seq(root1) == TreeNode::leaf_value_seq(root2)
            }
            (None, None) => true,
            _ => false,
        }
    }
}
