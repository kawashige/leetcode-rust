pub struct Solution {}

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![0; 1001];

        for num in &nums {
            for n in num {
                count[*n as usize] += 1;
            }
        }

        (1..=1000)
            .filter(|i| count[*i as usize] == nums.len())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2248() {
        assert_eq!(
            Solution::intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![]
        );
    }
}
