pub struct Solution {}

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; 201];

        for num in &nums {
            freq[(num + 100) as usize] += 1;
        }

        nums.sort_unstable_by(|a, b| {
            freq[(a + 100) as usize]
                .cmp(&freq[(b + 100) as usize])
                .then(b.cmp(&a))
        });
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1636() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
        assert_eq!(
            Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
            vec![1, 3, 3, 2, 2]
        );
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
