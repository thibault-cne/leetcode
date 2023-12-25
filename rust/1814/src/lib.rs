pub struct Solution;

impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let rev = |mut n: i32| { let mut r = 0; while n > 0 { r = r * 10 + (n % 10); n /= 10 } r };

        nums
            .iter()
            .map(|&x| x - rev(x))
            .for_each(|x| *map.entry(x).or_insert(0) += 1);

        map
            .values()
            .fold(0, |acc, &v| (acc + v * (v - 1) / 2) % Solution::MOD) as _
    }

    pub fn count_nice_pairs_naive(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let rev_i = format!("{}", nums[i])
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                let rev_j = format!("{}", nums[j])
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                ans += ((nums[i] + rev_j) == (nums[j] + rev_i)) as i64;
                ans %= Solution::MOD;
            }
        }

        ans as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
