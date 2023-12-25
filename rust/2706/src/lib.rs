pub struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (min_1, min_2) = prices.iter().fold((i32::MAX, i32::MAX), |acc, &v| {
            if v < acc.0 {
                (v, acc.0)
            } else if v < acc.1 {
                (acc.0, v)
            } else {
                acc
            }
        });

        if min_1 + min_2 <= money {
            money - (min_1 + min_2)
        } else {
            money
        }
    }
}
