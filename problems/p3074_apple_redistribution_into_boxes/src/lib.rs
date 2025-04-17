pub struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        capacity.sort_unstable_by_key(|c| -c);
        let mut remains = apple.into_iter().sum::<i32>();
        for i in 0..capacity.len() {
            if remains <= capacity[i] {
                return i as i32 + 1;
            }
            remains -= capacity[i];
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3074() {
        assert_eq!(
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
            2
        );
        assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
}
