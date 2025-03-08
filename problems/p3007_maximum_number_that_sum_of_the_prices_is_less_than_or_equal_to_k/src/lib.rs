pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, k: i64, x: i64) -> bool {
        let mut count: i64 = 0;
        let mut upper = 0;
        for i in (0..63).rev() {
            if (i + 1) % x == 0 {
                count = count.saturating_add(upper * 2_i64.pow(i as u32));
                if mid & 1 << i != 0 {
                    upper = upper * 2 + 1;
                    count = count.saturating_add(mid - upper * 2_i64.pow(i as u32) + 1);
                } else {
                    upper *= 2;
                }
            } else {
                upper *= 2;
                if mid & 1 << i != 0 {
                    upper += 1;
                }
            }
        }
        count <= k
    }

    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let mut ok = 0;
        let mut ng = std::i64::MAX;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, k, x as i64) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3007() {
        assert_eq!(Solution::find_maximum_number(1, 8), 128);
        assert_eq!(Solution::find_maximum_number(9, 1), 6);
        assert_eq!(Solution::find_maximum_number(7, 2), 9);
    }
}
