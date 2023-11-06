pub struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        let mut sort = (0..capacity.len())
            .map(|i| (capacity[i] - rocks[i]))
            .collect::<Vec<_>>();

        sort.sort_unstable();

        for i in 0..sort.len() {
            if additional_rocks < sort[i] {
                return i as i32;
            }

            additional_rocks -= sort[i];
        }

        capacity.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2),
            3
        );
        assert_eq!(
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100),
            3
        );
    }
}
