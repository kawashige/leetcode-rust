pub struct Solution {}

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![pivot; nums.len()];

        let mut i = 0;
        for num in &nums {
            if num < &pivot {
                result[i] = *num;
                i += 1;
            }
        }
        let mut i = nums.len() - 1;
        for num in nums.iter().rev() {
            if &pivot < num {
                result[i] = *num;
                i -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2161() {
        assert_eq!(
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
        assert_eq!(
            Solution::pivot_array(vec![-3, 4, 3, 2], 2),
            vec![-3, 2, 4, 3]
        );
    }
}
