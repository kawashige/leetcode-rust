pub struct Solution {}

impl Solution {
    pub fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }

    pub fn is_ok(
        mid: i64,
        divisor1: i64,
        divisor2: i64,
        unique_cnt1: i64,
        unique_cnt2: i64,
        div_lcm: i64,
    ) -> bool {
        let count_div1 = mid / divisor1;
        let count_div2 = mid / divisor2;
        let count_div1_and_dic2 = mid / div_lcm;

        let unique_cnt1 = (unique_cnt1 - (count_div2 - count_div1_and_dic2)).max(0);
        let unique_cnt2 = (unique_cnt2 - (count_div1 - count_div1_and_dic2)).max(0);

        unique_cnt1 + unique_cnt2 <= mid - (count_div1 + count_div2 - count_div1_and_dic2)
    }

    pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        let mut ok = 10_000_000_000;
        let mut ng = 1;
        let div_lcm = Self::lcm(divisor1 as i64, divisor2 as i64);

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(
                mid,
                divisor1 as i64,
                divisor2 as i64,
                unique_cnt1 as i64,
                unique_cnt2 as i64,
                div_lcm,
            ) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2513() {
        assert_eq!(Solution::minimize_set(2, 7, 1, 3), 4);
        assert_eq!(Solution::minimize_set(3, 5, 2, 1), 3);
        assert_eq!(Solution::minimize_set(2, 4, 8, 2), 15);
    }
}
