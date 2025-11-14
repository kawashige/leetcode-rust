pub struct Solution {}

impl Solution {
    pub fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
        let mut maximum_height = maximum_height;
        maximum_height.sort_unstable();
        let mut h = *maximum_height.last().unwrap();
        let mut result = h as i64;
        for i in (0..maximum_height.len() - 1).rev() {
            if h - 1 == 0 {
                return -1;
            }

            h = (h - 1).min(maximum_height[i]);
            result += h as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3301() {
        assert_eq!(Solution::maximum_total_sum(vec![2, 3, 4, 3]), 10);
        assert_eq!(Solution::maximum_total_sum(vec![15, 10]), 25);
        assert_eq!(Solution::maximum_total_sum(vec![2, 2, 1]), -1);
    }
}
