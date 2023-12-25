pub struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        paths
            .iter()
            .map(|v| &v[1])
            .find(|dst| !paths.iter().any(|v| v[0].as_str() == dst.as_str()))
            .unwrap()
            .to_string()
    }
}
