#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_vec(self) -> Vec<i32> {
        let mut vec = vec![];
        let mut node = Some(Box::new(self));
        while let Some(n) = node {
            vec.push(n.val);
            node = n.next;
        }
        vec
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(vec: Vec<i32>) -> Self {
        let mut head = ListNode::new(vec[0]);
        let mut tail = &mut head;
        for i in 1..vec.len() {
            let node = ListNode::new(vec[i]);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
        }
        head
    }
}

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let vec = head.unwrap().to_vec();
        let (mut i, mut j) = (0, vec.len() - 1);

        while i < j {
            if vec[i] != vec[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = ListNode::from(vec![1, 2, 2, 1]);
        let list_2 = ListNode::from(vec![1, 2, 3, 2, 1]);
        let list_3 = ListNode::from(vec![1, 2, 3, 3, 2, 1]);
        let list_4 = ListNode::from(vec![1, 2]);
        
        assert!(Solution::is_palindrome(Some(Box::new(list))));
        assert!(Solution::is_palindrome(Some(Box::new(list_2))));
        assert!(Solution::is_palindrome(Some(Box::new(list_3))));
        assert!(!Solution::is_palindrome(Some(Box::new(list_4))));
    }
}
