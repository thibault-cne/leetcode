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
}

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn range_sum_bst(root: Option<Node>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let mut ans = 0;
            let mut queue = Vec::new();

            queue.push(node);

            while let Some(node) = queue.pop() {
                if (low..=high).contains(&node.borrow().val) {
                    ans += node.borrow().val;
                }

                if let Some(left) = node.borrow().left.clone() {
                    queue.push(left);
                }

                if let Some(right) = node.borrow().right.clone() {
                    queue.push(right);
                }
            }

            ans
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 18,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
    }
}
