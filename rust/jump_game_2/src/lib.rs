const MAX_JUMPS: i32 = 10001;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // Self::jump_req(&nums, &mut vec![MAX_JUMPS; nums.len()], 0)
        Self::jump_bst(nums)
    }

    pub fn jump_bst(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut jumps = 0;
        let mut last_jump = 0;
        let mut max_reached = 0;

        loop {
            if last_jump as usize >= nums.len() - 1 {
                break;
            }

            max_reached = std::cmp::max(max_reached, i + nums[i as usize]);

            if i == last_jump {
                last_jump = max_reached;
                jumps += 1;
            }

            i += 1;
        }

        jumps
    }

    pub fn jump_req(nums: &[i32], memoization: &mut [i32], pos: usize) -> i32 {
        // base case
        if pos >= nums.len() - 1 {
            return 0;
        }

        if pos + nums[pos] as usize >= nums.len() - 1 {
            return 1;
        }

        if memoization[pos] != MAX_JUMPS {
            return memoization[pos];
        }

        (1..=nums[pos])
            .filter(|v| (*v as usize + pos) < nums.len() && nums[*v as usize + pos] > 0)
            .for_each(|v| {
                let min = std::cmp::min(
                    *memoization.get(pos).unwrap(),
                    1 + Self::jump_req(nums, memoization, v as usize + pos),
                );
                unsafe {
                    *memoization.get_unchecked_mut(pos) = min;
                }
            });

        memoization[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 2, 0, 1, 4]), 3);
        assert_eq!(Solution::jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]), 3);
    }
}
