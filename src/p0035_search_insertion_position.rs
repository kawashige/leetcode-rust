pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let max = nums.len() as i32;
        match nums.iter().position(|x| x >= &target) {
            Some(i) => i as i32,
            None => max,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_35() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }
}
