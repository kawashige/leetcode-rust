pub struct Solution {}

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut is_friend = vec![false; 101];
        for friend in friends {
            is_friend[friend as usize] = true;
        }
        order
            .into_iter()
            .filter(|o| is_friend[*o as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3668() {
        assert_eq!(
            Solution::recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]),
            vec![3, 1, 4]
        );
        assert_eq!(
            Solution::recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]),
            vec![5, 2]
        );
    }
}
