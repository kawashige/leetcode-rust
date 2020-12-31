pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..heights.len() {
            if 0 < i && heights[i - 1] == heights[i] {
                continue;
            }
            let left = (0..i)
                .rev()
                .take_while(|j| heights[i] <= heights[*j])
                .count();
            let right = ((i + 1)..heights.len())
                .take_while(|j| heights[i] <= heights[*j])
                .count();
            result = std::cmp::max(result, heights[i] * (left + right + 1) as i32)
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day31() {
        assert_eq!(0, Solution::largest_rectangle_area(vec![]));
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(
            100,
            Solution::largest_rectangle_area(vec![100, 1, 1, 1, 1, 1])
        );
    }
}
