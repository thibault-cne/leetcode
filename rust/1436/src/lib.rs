pub struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        paths
            .iter()
            .map(|v| &v[1])
            .find(|dst| paths.iter().map(|v| &v[0]).find(|src| src == dst).is_none())
            .unwrap()
            .to_string()
    }
}
