use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Cell = Rc<RefCell<TreeNode>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Cell>,
    pub right: Option<Cell>,
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
    pub fn tree2str(root: Option<Cell>) -> String {
        if let Some(root) = root {
            let root = root.borrow();
            match (root.left.clone(), root.right.clone()) {
                (Some(left), Some(right)) => format!(
                    "{}({})({})",
                    root.val,
                    Solution::tree2str(Some(left)),
                    Solution::tree2str(Some(right))
                ),
                (Some(left), None) => format!("{}({})", root.val, Solution::tree2str(Some(left)),),
                (None, Some(right)) => {
                    format!("{}()({})", root.val, Solution::tree2str(Some(right)))
                }
                (None, None) => format!("{}", root.val,),
            }
        } else {
            String::default()
        }
    }
}
