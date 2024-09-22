pub struct Solution {}

impl Solution {
    pub fn count_steps(n: i32, prefix1: i32, prefix2: i32) -> i32 {
        let mut prefix1 = prefix1 as i64;
        let mut prefix2 = prefix2 as i64;
        let n = n as i64;
        let mut steps = 0;

        while prefix1 <= n {
            steps += (n + 1).min(prefix2) - prefix1;
            prefix1 *= 10;
            prefix2 *= 10;
        }

        steps as i32
    }

    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut curr = 1;
        let mut k = k - 1;

        while 0 < k {
            let step = Self::count_steps(n, curr, curr + 1);
            if step <= k {
                curr += 1;
                k -= step;
            } else {
                curr *= 10;
                k -= 1;
            }
        }

        curr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0440() {
        assert_eq!(Solution::find_kth_number(1_000_000_000, 1_000_000_000), 17);
        assert_eq!(Solution::find_kth_number(100, 10), 17);
        assert_eq!(Solution::find_kth_number(2, 2), 2);
        assert_eq!(Solution::find_kth_number(13, 2), 10);
        assert_eq!(Solution::find_kth_number(1, 1), 1);
    }
}
