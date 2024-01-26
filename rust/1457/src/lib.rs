use std::cell::RefCell;
use std::rc::Rc;

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
}

pub struct Solution;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::pseudo_palindromic_paths_rec(root, &mut count, 0);
        count
    }

    fn pseudo_palindromic_paths_rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        count: &mut i32,
        mut path: u16,
    ) {
        if let Some(root) = root {
            let root = root.borrow();
            path ^= 1 << root.val;

            if root.left.is_none() && root.right.is_none() && path.count_ones() <= 1 {
                *count += 1;
                return;
            }

            Self::pseudo_palindromic_paths_rec(root.left.clone(), count, path);
            Self::pseudo_palindromic_paths_rec(root.right.clone(), count, path);
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));

        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    }
}
