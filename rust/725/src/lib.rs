pub type Node = Option<Box<ListNode>>;

// Definition for singly-linked list.
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

    fn slice_to(head: &mut Node, a: usize) -> Node {
        let mut current = head;
        let mut cnt = 1;

        loop {
            if current.is_none() || cnt == a {
                break;
            }

            cnt += 1;
            current = &mut current.as_mut().unwrap().next;
        }

        if current.is_some() {
            let tail = current.as_mut().unwrap().next.take();
            return tail;
        }

        None
    }
}

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut vec = vec![None; k as usize];
        let mut sizes_vec = vec![0; k as usize];
        let mut index = 0;

        let mut current = head.clone();

        loop {
            if current.is_none() {
                break;
            }

            if let Some(size) = sizes_vec.get_mut(index) {
                *size += 1;
            }

            if index == k as usize - 1 {
                index = 0;
            } else {
                index += 1;
            }

            current = current.unwrap().next;
        }

        let mut current = head;

        sizes_vec.iter().enumerate().for_each(|(i, s)| {
            let tail = ListNode::slice_to(&mut current, *s);
            vec[i] = current.clone();
            current = tail;
        });

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let expected = vec![
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(2))),
            Some(Box::new(ListNode::new(3))),
            None,
            None,
        ];

        assert_eq!(Solution::split_list_to_parts(Some(Box::new(list)), 5), expected);
    }

    #[test]
    fn empty() {
        let expected = vec![None, None, None, None, None];

        assert_eq!(Solution::split_list_to_parts(None, 5), expected);
    }

    #[test]
    fn no_split() {
        let list = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let expected = vec![Some(Box::new(list.clone()))];

        assert_eq!(Solution::split_list_to_parts(Some(Box::new(list)), 1), expected);
    }
}
