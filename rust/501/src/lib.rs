use std::cell::RefCell;
use std::collections::HashMap;
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

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut modes = Vec::new();
        let mut curr_value = i32::MIN;
        let mut curr_count = 0;
        let mut max = 0;

        Self::in_order_traversal(&root, &mut curr_value, &mut curr_count, &mut max, &mut modes);

        modes
    }

    pub fn in_order_traversal(node: &Option<Rc<RefCell<TreeNode>>>, curr_value: &mut i32, curr_count: &mut i32, max: &mut i32, modes: &mut Vec<i32>) {
        if let Some(x) = node {
            Self::in_order_traversal(&x.borrow().left, curr_value, curr_count, max, modes);

            if *curr_value == x.borrow().val {
                *curr_count += 1;
            } else {
                *curr_value = x.borrow().val;
                *curr_count = 1;
            }

            match curr_count.cmp(&max) {
                std::cmp::Ordering::Greater => {
                    *max = *curr_count;
                    modes.clear();
                    modes.push(*curr_value);
                },
                std::cmp::Ordering::Equal => {
                    modes.push(*curr_value);
                },
                _ => {}
                
            }

            Self::in_order_traversal(&x.borrow().right, curr_value, curr_count, max, modes);
        }
    }
        
    pub fn find_mode_naive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        Self::find_mode_rec_naive(root, &mut map);

        let max = map.values().fold(i32::MIN, |a, b| a.max(*b));

        map.iter()
            .filter(|(_, v)| **v == max)
            .map(|(k, _)| *k)
            .collect()
    }

    fn find_mode_rec_naive(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) {
        if let Some(x) = root {
            let val = x.borrow().val;
            *map.entry(val).or_insert(0) += 1;

            Self::find_mode_rec_naive(x.borrow().left.clone(), map);
            Self::find_mode_rec_naive(x.borrow().right.clone(), map);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        })));

        assert_eq!(Solution::find_mode_naive(root.clone()), vec![2]);
        assert_eq!(Solution::find_mode(root), vec![2]);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));

        assert_eq!(Solution::find_mode_naive(root.clone()), vec![1, 2]);
        assert_eq!(Solution::find_mode(root), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(Solution::find_mode_naive(root.clone()), vec![0]);
        assert_eq!(Solution::find_mode(root), vec![0]);
    }
}