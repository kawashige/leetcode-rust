pub struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut nums = vec![0; mat[0].len()];
        let mut count = 0;

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                nums[j] = if mat[i][j] == 0 {
                    0
                } else {
                    nums[j] + mat[i][j]
                };
            }

            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    continue;
                }
                let mut min = std::i32::MAX;
                for k in j..mat[0].len() {
                    min = min.min(nums[k]);
                    count += min;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1504() {
        assert_eq!(
            Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            13
        );
        assert_eq!(
            Solution::num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]),
            24
        );
    }
}
