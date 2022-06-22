pub struct Solution {}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut sums = Vec::with_capacity((n * (n + 1) / 2) as usize);

        for i in 0..n as usize {
            let mut sum = 0;
            for j in i..n as usize {
                sum += nums[j];
                sums.push(sum);
            }
        }

        sums.sort_unstable();
        sums[left as usize - 1..right as usize]
            .iter()
            .fold(0, |acc, x| (acc + x) % M)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1508() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
    }
}
