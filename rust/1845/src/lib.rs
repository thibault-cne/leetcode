use std::collections::BinaryHeap;

pub struct SeatManager {
    n: i32,
    available: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    pub fn new(n: i32) -> Self {
        Self {
            n,
            available: (1..=n).collect(),
        }
    }

    pub fn reserve(&mut self) -> i32 {
        (self.n + 1) - self.available.pop().unwrap()
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        self.available.push((self.n + 1) - seat_number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = SeatManager::new(5);
        assert_eq!(obj.reserve(), 1);
        assert_eq!(obj.reserve(), 2);
        obj.unreserve(2);
        assert_eq!(obj.reserve(), 2);
        assert_eq!(obj.reserve(), 3);
        assert_eq!(obj.reserve(), 4);
        assert_eq!(obj.reserve(), 5);
        obj.unreserve(5);
    }
}