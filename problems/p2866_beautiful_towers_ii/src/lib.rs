pub struct Solution {}

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut left = vec![max_heights[0] as i64; max_heights.len()];
        let mut stack = vec![(max_heights[0], 1)];
        let mut sum = max_heights[0] as i64;
        for i in 1..max_heights.len() {
            let mut count = 1;
            while !stack.is_empty() && max_heights[i] <= stack.last().unwrap().0 {
                let (h, c) = stack.pop().unwrap();
                sum -= h as i64 * c;
                count += c;
            }
            sum += max_heights[i] as i64 * count;
            stack.push((max_heights[i], count));
            left[i] = sum;
        }

        let mut result = left[left.len() - 1];

        let mut stack = vec![(max_heights[max_heights.len() - 1], 1)];
        let mut sum = max_heights[max_heights.len() - 1] as i64;
        for i in (0..max_heights.len() - 1).rev() {
            let mut count = 1;
            while !stack.is_empty() && max_heights[i] <= stack.last().unwrap().0 {
                let (h, c) = stack.pop().unwrap();
                sum -= h as i64 * c;
                count += c;
            }
            sum += max_heights[i] as i64 * count;
            stack.push((max_heights[i], count));
            result = result.max(left[i] + sum - max_heights[i] as i64);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2866() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }
}
