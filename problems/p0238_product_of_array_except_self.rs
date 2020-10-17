pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut results = Vec::new();

        let mut p = 1;
        results.push(p);
        for i in 0..(nums.len() - 1) {
            p *= nums[i];
            results.push(p);
        }
        p = 1;
        for i in (1..nums.len()).rev() {
            p *= nums[i];
            results[i - 1] *= p;
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0238() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
        assert_eq!(vec![3, 2], Solution::product_except_self(vec![2, 3]));
    }
}
