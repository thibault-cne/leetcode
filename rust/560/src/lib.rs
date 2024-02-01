pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = Vec::with_capacity(nums.len());
        let mut map = std::collections::HashMap::new();

        prefix.push(nums[0]);

        nums.iter().enumerate().skip(1).for_each(|(i, &n)| {
            prefix.push(prefix[i - 1] + n);
        });

        (0..nums.len()).fold(0, |mut acc, i| {
            if prefix[i] == k {
                acc += 1;
            }

            if let Some(&v) = map.get(&(prefix[i] - k)) {
                acc += v;
            }

            *map.entry(prefix[i]).or_insert(0) += 1;

            acc
        })
    }
}
