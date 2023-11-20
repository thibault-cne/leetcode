pub struct Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for v in adjacent_pairs {
            map.entry(v[0]).or_insert(Vec::new()).push(v[1]);
            map.entry(v[1]).or_insert(Vec::new()).push(v[0]);
        }

        let mut prev = i32::MIN;
        let mut curr = *map
            .iter()
            .find(|(_, v)| v.len() == 1)
            .map(|(k, _)| k)
            .unwrap();

        let mut res = Vec::new();
        loop {
            res.push(curr);

            if let Some(&next) = map[&curr].iter().find(|&&n| n != prev) {
                prev = curr;
                curr = next;
            } else {
                break;
            }
        }

        res
    }
}
