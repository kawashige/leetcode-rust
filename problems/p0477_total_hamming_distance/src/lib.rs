pub struct Solution {}

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ones = [0; 31];
        let l = nums.len();

        for n in nums {
            for i in 0_usize..31 {
                if 0 < n & (1 << i) {
                    ones[i] += 1;
                }
            }
        }

        ones.iter().map(|o| (o * (l - o)) as i32).sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0477() {
        assert_eq!(6, Solution::total_hamming_distance(vec![4, 14, 2]));
        assert_eq!(1, Solution::total_hamming_distance(vec![0, 1]));
        assert_eq!(0, Solution::total_hamming_distance(vec![]));
    }
}
