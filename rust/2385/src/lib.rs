use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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

    fn into_graph(root: Rc<RefCell<TreeNode>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = Vec::new();

        queue.push(root);

        while let Some(node) = queue.pop() {
            let node = node.borrow();

            if let Some(left) = node.left.clone() {
                graph.entry(node.val).or_default().push(left.borrow().val);
                graph.entry(left.borrow().val).or_default().push(node.val);
                queue.push(left);
            }

            if let Some(right) = node.right.clone() {
                graph.entry(node.val).or_default().push(right.borrow().val);
                graph.entry(right.borrow().val).or_default().push(node.val);
                queue.push(right);
            }
        }

        graph
    }
}

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let graph = TreeNode::into_graph(root.unwrap());
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();

        dist.insert(start, 0);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if let Some(adjacents) = graph.get(&node) {
                adjacents.iter().for_each(|&a| {
                    if !dist.contains_key(&a) {
                        dist.insert(a, dist.get(&node).unwrap() + 1);
                        queue.push_back(a);
                    }
                });
            }
        }

        dist.into_values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        assert_eq!(
            Solution::amount_of_time(Some(Rc::new(RefCell::new(root))), 3),
            4
        );
    }
}
