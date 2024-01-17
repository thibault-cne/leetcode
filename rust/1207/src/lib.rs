pub struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut set = std::collections::HashSet::new();

        arr.into_iter()
            .for_each(|v| *map.entry(v).or_insert(0) += 1);

        for v in map.into_values() {
            if !set.insert(v) {
                return false;
            }
        }

        true
    }
}
