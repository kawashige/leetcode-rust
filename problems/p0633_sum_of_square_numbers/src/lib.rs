pub struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut c = c;
        let mut i: i32 = 2;
        while i.saturating_mul(i) <= c {
            let mut count = 0;
            if c % i == 0 {
                while c % i == 0 {
                    count += 1;
                    c /= i;
                }
                if i % 4 == 3 && count % 2 != 0 {
                    return false;
                }
            }
            i += 1;
        }
        c % 4 != 3
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0633() {
        assert!(!Solution::judge_square_sum(15));
        assert!(Solution::judge_square_sum(5));
        assert!(!Solution::judge_square_sum(3));
        assert!(Solution::judge_square_sum(4));
        assert!(Solution::judge_square_sum(2));
        assert!(Solution::judge_square_sum(1));
    }
}
