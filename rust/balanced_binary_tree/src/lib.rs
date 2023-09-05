use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn height(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 0,
            (Some(left), None) => 1 + left.borrow().height(),
            (None, Some(right)) => 1 + right.borrow().height(),
            (Some(left), Some(right)) => {
                1 + std::cmp::max(left.borrow().height(), right.borrow().height())
            }
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            match (&root.left, &root.right) {
                (None, None) => true,
                (Some(left), None) => left.borrow().height() < 1,
                (None, Some(right)) => right.borrow().height() < 1,
                (Some(left), Some(right)) => {
                    let diff = left.borrow().height() as isize - right.borrow().height() as isize;
                    diff.abs() < 2
                        && Self::is_balanced(Some(Rc::clone(left)))
                        && Self::is_balanced(Some(Rc::clone(right)))
                }
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let valid = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        };
        let valid_2 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        };

        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(valid)))));
        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(valid_2)))));
    }

    #[test]
    fn invalid() {
        let invalid = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        };

        assert!(!Solution::is_balanced(Some(Rc::new(RefCell::new(invalid)))));
    }
}
