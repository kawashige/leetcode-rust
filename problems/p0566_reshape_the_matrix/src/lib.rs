pub struct Solution {}

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if r * c != (nums.len() * nums[0].len()) as i32 {
            return nums;
        }

        let mut result = vec![Vec::with_capacity(c as usize); r as usize];
        let mut k = 0;
        for i in 0..nums.len() {
            for j in 0..nums[0].len() {
                if result[k].len() == c as usize {
                    k += 1;
                }
                result[k].push(nums[i][j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0566() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1]], 1, 1), vec![vec![1]]);
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
    }
}
