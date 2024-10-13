pub struct Solution {}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = vec![0; nums.len()];
        let mut all_nums: Vec<(i32, usize)> = nums
            .into_iter()
            .zip(0..)
            .map(|(num, i)| num.into_iter().map(|v| (v, i)).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        all_nums.sort_unstable();

        let mut result = vec![all_nums[0].0, all_nums[all_nums.len() - 1].0];
        let mut count = 0;
        let mut l = 0;
        for i in 0..all_nums.len() {
            counts[all_nums[i].1] += 1;
            if counts[all_nums[i].1] == 1 {
                count += 1;
            }
            if count == counts.len() {
                while 1 < counts[all_nums[l].1] {
                    counts[all_nums[l].1] -= 1;
                    l += 1;
                }
                if all_nums[i].0 - all_nums[l].0 < result[1] - result[0] {
                    result = vec![all_nums[l].0, all_nums[i].0];
                }
            }
        }
        vec![result[0] as i32, result[1] as i32]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0632() {
        assert_eq!(
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30]
            ]),
            vec![20, 24]
        );
        assert_eq!(
            Solution::smallest_range(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]),
            vec![1, 1]
        );
    }
}
