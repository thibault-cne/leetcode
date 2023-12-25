pub struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut set = std::collections::HashSet::new();
        let mut current = (0, 0);

        set.insert(current);

        for c in path.chars() {
            match c {
                'N' => current.0 += 1,
                'E' => current.1 += 1,
                'S' => current.0 -= 1,
                'W' => current.1 -= 1,
                _ => unreachable!(),
            }

            if !set.insert(current) {
                return true;
            }
        }

        false
    }
}
