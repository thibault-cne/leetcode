pub type OptNode = Option<Box<ListNode>>;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn slice_to(head: &mut OptNode, a: usize) -> OptNode {
        if a == 1 {
            let res = head.clone();
            head.take();
            return res;
        }

        let mut current = head;
        let mut cnt = 1;

        loop {
            if current.is_none() || cnt == a - 1 {
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

    fn reverse(head: OptNode) -> OptNode {
        let mut previous: OptNode = None;
        let mut current = head;

        loop {
            if current.is_none() {
                break;
            }

            let mut temp = current.clone();

            if let Some(tmp) = temp.as_mut() {
                tmp.next = previous;
            }

            previous = temp;
            current = current.unwrap().next;
        }

        previous
    }

    fn append(&mut self, tail: OptNode) {
        let mut current = self;

        loop {
            if current.next.is_none() {
                current.next = tail;
                break;
            }

            current = current.next.as_mut().unwrap();
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let unchanged_tail = ListNode::slice_to(&mut head, right as usize + 1);
        let tail = ListNode::slice_to(&mut head, left as usize);
        let rev = ListNode::reverse(tail);

        let mut head = if head.is_some() {
            let mut head = head.unwrap();
            head.append(rev);

            head
        } else {
            rev.unwrap()
        };

        head.append(unchanged_tail);

        Some(head)
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
    fn reverse() {
        let list = list![1, 2, 3];
        let expect = list![3, 2, 1];

        assert_eq!(ListNode::reverse(Some(boxed!(list))), Some(boxed!(expect)));
    }

    #[test]
    fn test() {
        let list = list![1, 2, 3, 4, 5];
        let expect = list![1, 4, 3, 2, 5];

        assert_eq!(
            Solution::reverse_between(Some(boxed!(list)), 2, 4),
            Some(boxed!(expect))
        );

        let list = list![5];
        let expect = list![5];

        assert_eq!(
            Solution::reverse_between(Some(boxed!(list)), 1, 1),
            Some(boxed!(expect))
        );

        let list = list![3, 5];
        let expect = list![5, 3];

        assert_eq!(
            Solution::reverse_between(Some(boxed!(list)), 1, 2),
            Some(boxed!(expect))
        );
    }
}
