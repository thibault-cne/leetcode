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

    fn min_max_in_subtree(&self) -> Option<(i32, i32)> {
        if self.left.is_none() && self.right.is_none() {
            return None;
        }

        let mut queue = Vec::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        if let Some(left) = self.left.clone() {
            queue.push(left);
        }

        if let Some(right) = self.right.clone() {
            queue.push(right);
        }

        while let Some(node) = queue.pop() {
            min = node.borrow().val.min(min);
            max = node.borrow().val.max(max);

            if let Some(left) = node.borrow().left.clone() {
                queue.push(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push(right);
            }
        }

        Some((min, max))
    }
}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();

        let mut ans = i32::MIN;
        let mut queue = vec![root];

        while let Some(node) = queue.pop() {
            if let Some((min, max)) = node.borrow().min_max_in_subtree() {
                ans = ans.max((max - node.borrow().val).abs());
                ans = ans.max((min - node.borrow().val).abs());
            }

            if let Some(left) = node.borrow().left.clone() {
                queue.push(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push(right);
            }
        }

        ans
    }
}
