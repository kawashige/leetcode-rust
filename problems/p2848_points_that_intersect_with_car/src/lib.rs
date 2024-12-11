pub struct Solution {}

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut line = vec![0; 102];
        for i in 0..nums.len() {
            line[nums[i][0] as usize] += 1;
            line[nums[i][1] as usize + 1] -= 1;
        }
        let mut result = 0;
        for i in 1..line.len() {
            line[i] += line[i - 1];
            if 0 < line[i] {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2848() {
        assert_eq!(
            Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
            7
        );
        assert_eq!(Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]), 7);
    }
}
