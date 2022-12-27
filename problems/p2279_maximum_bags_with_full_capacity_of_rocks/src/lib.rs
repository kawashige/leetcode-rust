pub struct Solution {}

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains = capacity
            .iter()
            .zip(rocks.iter())
            .map(|(c, r)| c - r)
            .collect::<Vec<_>>();
        remains.sort_unstable();
        let mut sum = 0;

        for i in 0..rocks.len() {
            sum += remains[i];
            if additional_rocks < sum {
                return i as i32;
            }
        }

        remains.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2279() {
        assert_eq!(
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2),
            3
        );
        assert_eq!(
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100),
            3
        );
    }
}
