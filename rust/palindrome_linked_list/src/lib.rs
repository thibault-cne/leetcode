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

    pub fn reverse(&self) -> Self {
        let mut head = ListNode {
            next: None,
            val: self.val,
        };
        let mut current = self.next;

        loop {
            if current.is_none() {
                break;
            }

            head = ListNode {
                next: Some(head),
                val: (*current.unwrap()).value,
            };
            current = (*current.unwrap()).next;
        }

        head
    }
}

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {}
}
