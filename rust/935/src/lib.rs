pub struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        if n == 1 {
            return 10;
        }

        const MOD: u32 = 1e9 as u32 + 7;
        ((1..n)
            .fold([1, 4, 2, 2], |v, _| {
                [
                    v[3],
                    ((v[2] + v[3]) << 1) % MOD,
                    v[1],
                    ((v[0] << 1) + v[1]) % MOD,
                ]
            })
            .into_iter()
            .sum::<u32>()
            % MOD) as i32
    }
}
