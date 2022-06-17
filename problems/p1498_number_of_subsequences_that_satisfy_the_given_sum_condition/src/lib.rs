pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;
    pub fn pow(i: usize) -> usize {
        if i == 0 {
            1
        } else if i % 2 == 0 {
            let x = Self::pow(i / 2);
            x * x % Self::M
        } else {
            2 * Self::pow(i - 1) % Self::M
        }
    }

    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;

        nums.sort_unstable();

        let mut r = nums.len() - 1;

        for i in 0..nums.len() {
            r = i.max(r);
            while i < r && target < nums[i] + nums[r] {
                r -= 1;
            }

            if nums[i] + nums[r] <= target {
                count = (count + Self::pow(r - i)) % Self::M;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1498() {
        assert_eq!(
            Solution::num_subseq(
                vec![
                    14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15,
                    13, 11, 10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15,
                    19, 14
                ],
                22
            ),
            272187084
        );
        assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
        assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
        assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    }
}
