pub struct Solution {}

impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let mut pre_l = vec![0; s.len() + 1];
        let mut pre_lc = vec![0; s.len() + 1];
        for i in 0..s.len() {
            pre_l[i + 1] = pre_l[i];
            pre_lc[i + 1] = pre_lc[i];
            match s.as_bytes()[i] {
                b'L' => pre_l[i + 1] += 1,
                b'C' => pre_lc[i + 1] += pre_l[i],
                _ => {}
            };
        }

        let mut suf_t = vec![0; s.len() + 1];
        let mut suf_ct = vec![0; s.len() + 1];
        for i in (0..s.len()).rev() {
            suf_t[i] = suf_t[i + 1];
            suf_ct[i] = suf_ct[i + 1];
            match s.as_bytes()[i] {
                b'T' => suf_t[i] += 1,
                b'C' => suf_ct[i] += suf_t[i + 1],
                _ => {}
            };
        }

        let mut base = 0;
        let mut max_gain = 0;

        for i in 0..s.len() + 1 {
            if 0 < i && s.as_bytes()[i - 1] == b'C' {
                base += pre_l[i] * suf_t[i];
            }
            max_gain = max_gain
                .max(suf_ct[i])
                .max(pre_lc[i])
                .max(suf_t[i] * pre_l[i]);
        }

        base + max_gain
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3628() {
        assert_eq!(Solution::num_of_subsequences("LMCT".to_string()), 2);
        assert_eq!(Solution::num_of_subsequences("LCCT".to_string()), 4);
        assert_eq!(Solution::num_of_subsequences("L".to_string()), 0);
    }
}
