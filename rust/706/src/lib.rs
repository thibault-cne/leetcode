pub struct MyHashMap {
    buckets: Vec<Option<Vec<(i32, i32)>>>,
}

// Prime number to reduce collisions
const SIZE: usize = 1031;
const NONE: Option<Vec<(i32, i32)>> = None;

impl MyHashMap {
    
    pub fn new() -> Self {
        Self {
            buckets: vec![NONE; SIZE],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % SIZE
    }

    fn index_in_bucket(&mut self, key: i32) -> (&mut Vec<(i32, i32)>, Result<usize, usize>) {
        let bucket = &mut self.buckets[Self::hash(key)];
        if bucket.is_none() {
            *bucket = Some(Vec::new());
        }
        let result = bucket.as_ref().unwrap().binary_search_by(|(k, _)| k.cmp(&key));
        (bucket.as_mut().unwrap(), result)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        match self.index_in_bucket(key) {
            (bucket, Ok(idx)) => bucket[idx].1 = value,
            (bucket, Err(idx)) => bucket.insert(idx, (key, value)),
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let bucket = &self.buckets[Self::hash(key)];
        if bucket.is_none() {
            - 1
        } else {
            match bucket.as_ref().unwrap().binary_search_by(|(k, _)| k.cmp(&key)) {
                Ok(idx) => bucket.as_ref().unwrap()[idx].1,
                Err(_) => -1,
            }
        }
    }

    pub fn remove(&mut self, key: i32) {
        match self.index_in_bucket(key) {
            (bucket, Ok(idx)) => { bucket.remove(idx); }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }

    #[test]
    fn test_2() {
        // Long chain of collisions
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        map.put(1032, 3);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(2), 2);
        assert_eq!(map.get(1032), 3);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }
}