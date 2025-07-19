pub struct Solution {}

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut occurences = vec![-1; 100_001];
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] == x {
                j += 1;
                occurences[j] = i as i32;
            }
        }

        queries
            .into_iter()
            .map(|q| occurences[q as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3159() {
        assert_eq!(
            Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
            vec![0, -1, 2, -1]
        );
        assert_eq!(
            Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5),
            vec![-1]
        );
    }
}
