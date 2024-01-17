pub struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = std::collections::HashMap::<i32, i32>::new();

        matches.iter().for_each(|m| {
            map.entry(m[0]).or_insert(0);
            *map.entry(m[1]).or_insert(0) += 1;
        });

        let mut winners: Vec<i32> = map
            .iter()
            .filter(|(_, l)| **l == 0)
            .map(|(k, _)| *k)
            .collect();
        let mut one_loose: Vec<i32> = map
            .iter()
            .filter(|(_, l)| **l == 1)
            .map(|(k, _)| *k)
            .collect();
        winners.sort_unstable();
        one_loose.sort_unstable();

        vec![winners, one_loose]
    }
}
