pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let mut res = vec![1];
        let prev = Self::get_row(row_index - 1);

        (0..prev.len() - 1).for_each(|i| res.push(prev[i] + prev[i + 1]));

        res.push(1);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
