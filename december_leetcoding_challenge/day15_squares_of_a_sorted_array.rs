pub struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        for n in nums {
            if n == 0 {
                result.push(0);
            } else if n < 0 {
                stack.push(n * n);
            } else {
                let tmp = n * n;
                while !stack.is_empty() && stack.last().unwrap() <= &tmp {
                    result.push(stack.pop().unwrap());
                }
                result.push(tmp);
            }
        }
        while !stack.is_empty() {
            result.push(stack.pop().unwrap());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day15() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
        assert_eq!(vec![100], Solution::sorted_squares(vec![10]));
        assert_eq!(vec![1, 16], Solution::sorted_squares(vec![-4, -1]));
        assert_eq!(vec![0, 1, 16], Solution::sorted_squares(vec![-4, -1, 0]));
    }
}
