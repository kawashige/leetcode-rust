pub struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        pub fn recurse(tokens: &[i32], p: i32, s: i32) -> i32 {
            if tokens.is_empty() {
                return s;
            }
            if tokens.iter().sum::<i32>() <= p {
                return s + tokens.len() as i32;
            }

            let mut results = vec![s];
            if tokens[0] < p {
                results.push(recurse(&tokens[1..], p - tokens[0], s + 1))
            } else if 0 < s && 2 < tokens.len() && tokens[0] < *tokens.last().unwrap() {
                let l = tokens.len() - 1;
                results.push(recurse(&tokens[..l], p + tokens[l], s - 1));
            }
            results.into_iter().max().unwrap()
        }
        let mut tokens = tokens;
        tokens.sort();
        recurse(&tokens, p, 0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day24() {
        assert_eq!(
            5,
            Solution::bag_of_tokens_score(vec![6, 0, 39, 52, 45, 49, 59, 68, 42, 37], 99)
        );
        assert_eq!(0, Solution::bag_of_tokens_score(vec![100], 50));
        assert_eq!(1, Solution::bag_of_tokens_score(vec![100, 200], 150));
        assert_eq!(
            2,
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
        );
        assert_eq!(
            20,
            Solution::bag_of_tokens_score(
                vec![
                    2897, 6861, 2070, 5292, 3402, 9784, 9718, 2089, 5660, 3294, 9685, 9245, 5861,
                    7200, 6813, 3533, 9163, 8994, 3306, 7473, 90, 8163, 5648, 9523, 3631, 6257,
                    3230, 7827, 6007, 9874, 10, 1407, 436, 1258, 9293, 9486, 4804, 9466, 8183,
                    7786, 7472, 1876, 5488, 4238, 9497, 1738, 1698, 6588, 1574, 1100
                ],
                5039
            )
        );
    }
}
