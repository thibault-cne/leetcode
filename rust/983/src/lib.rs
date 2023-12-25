pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![-1; *days.last().unwrap() as usize + 1];

        solve(&days, &costs, *days.first().unwrap(), &mut dp)
    }
}

fn solve(days: &[i32], costs: &[i32], day: i32, dp: &mut [i32]) -> i32 {
    let last_day = *days.last().unwrap();
    if day > last_day {
        return 0;
    }

    if !days.contains(&day) {
        return solve(days, costs, day + 1, dp);
    }

    if dp[day as usize] != -1 {
        return dp[day as usize];
    }

    let one_day_pass = costs[0] + solve(days, costs, day + 1, dp);
    let seven_day_pass = costs[1] + solve(days, costs, day + 7, dp);
    let thirty_day_pass = costs[2] + solve(days, costs, day + 30, dp);

    let min = one_day_pass.min(seven_day_pass.min(thirty_day_pass));

    dp[day as usize] = min;
    min
}
