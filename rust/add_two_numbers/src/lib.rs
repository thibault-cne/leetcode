// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut overflow = 0;
        let mut head: Option<Box<ListNode>> = None;
        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            let n1 = match l1 {
                Some(node) => {
                    l1 = node.next;
                    Some(node.val)
                }
                None => None,
            };

            let n2 = match l2 {
                Some(node) => {
                    l2 = node.next;
                    Some(node.val)
                }
                None => None,
            };

            let sum = match (n1, n2) {
                (Some(n1), Some(n2)) => n1 + n2 + overflow,
                (Some(n1), None) => n1 + overflow,
                (None, Some(n2)) => n2 + overflow,
                (None, None) => overflow,
            };

            if sum == 0 && l1.is_none() && l2.is_none() && head.is_some() {
                break;
            }

            let node = ListNode::new(sum % 10);

            match head {
                Some(ref mut head) => {
                    let mut tail = &mut head.next;
                    while let Some(n) = tail {
                        tail = &mut n.next;
                    }
                    *tail = Some(Box::new(node));
                }
                None => {
                    head = Some(Box::new(node));
                }
            }

            overflow = sum / 10;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! boxed {
        ($expr:expr) => {
            Box::new($expr)
        };
    }

    macro_rules! list {
        ($head:expr, $($tail:tt)*) => {
            ListNode {
                val: $head,
                next: list!(inner $($tail)*),
            }
        };
        ($head:expr) => {
            ListNode {
                val: $head,
                next: None,
            }
        };
        (inner $head:expr, $($tail:tt)*) => {
            Some(boxed!(ListNode {
                val: $head,
                next: list!(inner $($tail)*)
            }))
        };
        (inner $head:expr) => {
            Some(boxed!(ListNode {
                val: $head,
                next: None
            }))
        };
    }

    #[test]
    fn test_macro() {
        let list = list!(1, 2, 3);
        let expect = ListNode {
            val: 1,
            next: Some(boxed!(ListNode {
                val: 2,
                next: Some(boxed!(ListNode { val: 3, next: None }))
            })),
        };

        assert_eq!(list, expect);
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(boxed!(list!(2, 4, 3)));
        let l2 = Some(boxed!(list!(5, 6, 4)));
        let result = Some(boxed!(list!(7, 0, 8)));

        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = Some(boxed!(list!(9, 9, 9, 9, 9, 9, 9)));
        let l2 = Some(boxed!(list!(9, 9, 9, 9)));
        let result = Some(boxed!(list!(8, 9, 9, 9, 0, 0, 0, 1)));

        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = Some(boxed!(list!(9)));
        let l2 = Some(boxed!(list!(1, 9, 9, 9, 9, 9, 9, 9, 9, 9)));
        let result = Some(boxed!(list!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)));

        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = Some(boxed!(list!(0)));
        let l2 = Some(boxed!(list!(0)));
        let result = Some(boxed!(list!(0)));

        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }
}
