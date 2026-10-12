pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = [0; 32];
        for i in 0..nums.len() {
            for j in 0..32 {
                if nums[i] & 1 << j != 0 {
                    count[j] += 1;
                }
            }
        }

        let mut result = 0;
        const M: usize = 1_000_000_007;
        for _ in 0..k {
            let mut x = 0;
            for j in 0..32 {
                if 0 < count[j] {
                    x |= 1 << j;
                    count[j] -= 1;
                }
            }
            result = (result + (x * x) % M) % M;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2897() {
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }
}

fn main() {}
