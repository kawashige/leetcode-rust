pub struct Solution {}

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if (1..suits.len()).all(|i| suits[0] == suits[i]) {
            return "Flush".to_string();
        }
        let max_count = ranks
            .into_iter()
            .fold([0; 13], |mut count, r| {
                count[r as usize] += 1;
                count
            })
            .into_iter()
            .max()
            .unwrap();

        if 2 < max_count {
            "Three of a Kind".to_string()
        } else if max_count == 2 {
            "Pair".to_string()
        } else {
            "High Card".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2347() {
        assert_eq!(
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush".to_string()
        );
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind".to_string()
        );
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair".to_string()
        );
    }
}
