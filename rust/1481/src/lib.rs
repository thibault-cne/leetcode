pub struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut map = std::collections::HashMap::with_capacity(arr.len());

        arr.into_iter()
            .for_each(|x| *map.entry(x).or_insert(0) += 1);

        let mut v = map.into_values().collect::<Vec<_>>();
        v.sort_unstable();

        v.iter()
            .skip_while(|&count| {
                k -= count;
                k >= 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_least_num_of_unique_ints() {
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    }
}
