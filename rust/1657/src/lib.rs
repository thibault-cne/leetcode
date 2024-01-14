pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let freq = |s: String| {
            let mut f = [0; 26];
            s.bytes().for_each(|b| {
                let i = (b - b'a') as usize;
                *unsafe { f.get_unchecked_mut(i) } += 1;
            });
            f
        };

        let freq_1 = freq(word1);
        let freq_2 = freq(word2);

        if freq_1
            .iter()
            .zip(freq_2.iter())
            .any(|(&l, &r)| l == 0 && r > 0 || r == 0 && l > 0)
        {
            return false;
        }

        let mut freq_1 = freq_1;
        let mut freq_2 = freq_2;

        freq_1.sort_unstable();
        freq_2.sort_unstable();

        !freq_1.iter().zip(freq_2.iter()).any(|(&l, &r)| l != r)
    }
}
