pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..(nums.len() - k as usize % nums.len()) {
            let n = nums.remove(0);
            nums.push(n);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0189() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut input, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], input);

        let mut input = vec![-1, -100, 3, 99];
        Solution::rotate(&mut input, 2);
        assert_eq!(vec![3, 99, -1, -100], input);
    }
}
