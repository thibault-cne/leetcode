#![allow(clippy::new_without_default)]

use rand::seq::IteratorRandom;

pub struct RandomizedSet {
    inner: std::collections::HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.inner.insert(val)
    }

    pub fn remove(&mut self, val: i32) -> bool {
        self.inner.remove(&val)
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();

        *self.inner.iter().choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let rand = RandomizedSet::new();

        assert!(rand.inner.is_empty());
    }
}
