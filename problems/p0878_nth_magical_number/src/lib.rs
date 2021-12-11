pub struct Solution {}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const M: u64 = 1_000_000_007;

        let mut nums = Vec::new();
        let mut a_m = a as u64;
        let mut b_m = b as u64;
        while a_m != b_m {
            if a_m < b_m {
                nums.push(a_m);
                a_m += a as u64;
            } else {
                nums.push(b_m);
                b_m += b as u64;
            }
        }
        nums.push(a_m);

        let m = (n as u64 - 1) / nums.len() as u64;
        let r = n as usize % nums.len();

        ((*nums.last().unwrap() * m % M + nums[(r + nums.len() - 1) % nums.len()]) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0878() {
        assert_eq!(
            Solution::nth_magical_number(1000000000, 40000, 40000),
            999720007
        );
        assert_eq!(Solution::nth_magical_number(1, 2, 3), 2);
        assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
        assert_eq!(Solution::nth_magical_number(5, 2, 4), 10);
        assert_eq!(Solution::nth_magical_number(3, 6, 4), 8);
    }
}
