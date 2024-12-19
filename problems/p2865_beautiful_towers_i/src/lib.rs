pub struct Solution {}

impl Solution {
    pub fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
        let mut result = 0;

        for i in 0..heights.len() {
            let mut sum = heights[i] as i64;
            let mut h = heights[i];
            for j in (0..i).rev() {
                h = h.min(heights[j]);
                sum += h as i64;
            }
            h = heights[i];
            for j in i + 1..heights.len() {
                h = h.min(heights[j]);
                sum += h as i64;
            }
            result = result.max(sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2865() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }
}
