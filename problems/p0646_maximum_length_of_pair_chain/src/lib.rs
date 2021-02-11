pub struct Solution {}

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|a| a[1]);
        let mut count = 0;
        let mut current = std::i32::MIN;
        for pair in pairs {
            if current < pair[0] {
                current = pair[1];
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0645() {
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![-6, 9],
                vec![1, 6],
                vec![8, 10],
                vec![-1, 4],
                vec![-6, -2],
                vec![-9, 8],
                vec![-5, 3],
                vec![0, 3]
            ]),
            3
        );
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![9, 10],
                vec![-4, 9],
                vec![-5, 6],
                vec![-5, 9],
                vec![8, 9]
            ]),
            2
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }
}
