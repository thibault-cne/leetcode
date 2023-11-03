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

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Solution::average_of_subtree_rec(root, &mut count);
        count
    }

    pub fn average_of_subtree_rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        count: &mut i32,
    ) -> (i32, i32) {
        if let Some(x) = root {
            let x = x.borrow();

            let (left_count, left_nb) = Solution::average_of_subtree_rec(x.left.clone(), count);
            let (right_count, right_nb) = Solution::average_of_subtree_rec(x.right.clone(), count);

            let val = left_count + right_count + x.val;
            let n = left_nb + right_nb + 1;

            let average = val / n;

            if average == x.val {
                *count += 1;
            }

            (val, n)
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(Solution::average_of_subtree(root), 1);
    }

    #[test]
    fn test_2() {
        // root = [4,8,5,0,1,null,6]
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));

        assert_eq!(Solution::average_of_subtree(root), 5);
    }
}