pub struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let found = nums.into_iter().fold([false; 1001], |mut found, num| {
            found[num as usize] = true;
            found
        });
        let mut value = original as usize;
        loop {
            if found.len() <= value || !found[value] {
                return value as i32;
            }
            value *= 2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2154() {
        assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
        assert_eq!(Solution::find_final_value(vec![2, 7, 9], 4), 4);
    }
}
