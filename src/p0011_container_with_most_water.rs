pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..height.len() {
            for j in 0..height.len() {
                let area = (j - i) as i32 * std::cmp::min(height[i], height[j]);
                if max < area {
                    max = area
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_p0011() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
