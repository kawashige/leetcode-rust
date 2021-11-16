pub struct Solution {}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut ng = 0;
        let mut ok = m * n;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if k <= (1..=m).fold(0, |acc, i| acc + (mid / i).min(n)) {
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
    fn test_0668() {
        assert_eq!(Solution::find_kth_number(9895, 28045, 100787757), 31852588);
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }
}
