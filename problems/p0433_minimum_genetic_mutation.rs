pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        fn can_mutation(s1: &str, s2: &str) -> bool {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let mut is_same = true;
            for i in 0..b1.len() {
                if b1[i] != b2[i] {
                    if is_same {
                        is_same = false;
                    } else {
                        return false;
                    }
                }
            }
            true
        }

        let mut matrix = vec![vec![0; bank.len()]; bank.len()];
        for i in 0..bank.len() {
            for j in 0..bank.len() {
                if i != j && can_mutation(&bank[i], &bank[j]) {
                    matrix[i][j] = 1;
                }
            }
        }
        let mut nexts = (0..bank.len())
            .filter(|i| can_mutation(&start, &bank[*i]))
            .collect::<HashSet<usize>>();
        let end_pos = bank.iter().position(|b| b == &end);
        if end_pos.is_none() {
            return -1;
        }
        let end_pos = end_pos.unwrap();
        let mut opened = HashSet::new();
        let mut count = 0;
        println!(
            "nexts: {:?}, ends:{:?}, matrix: {:?}",
            nexts, end_pos, matrix
        );
        while !nexts.is_empty() {
            count += 1;
            let mut new_nexts = HashSet::new();
            for n in nexts {
                if n == end_pos {
                    return count;
                }
                if opened.contains(&n) {
                    continue;
                }
                opened.insert(n);
                for (nn, _) in matrix[n].iter().enumerate().filter(|(_, m)| *m == &1) {
                    if !opened.contains(&nn) {
                        new_nexts.insert(nn);
                    }
                }
            }
            nexts = new_nexts;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0433() {
        assert_eq!(
            -1,
            Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec![])
        );
        assert_eq!(
            1,
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AACCGGTA".to_string(),
                vec!["AACCGGTA".to_string()]
            )
        );
        assert_eq!(
            2,
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                vec![
                    "AACCGGTA".to_string(),
                    "AACCGCTA".to_string(),
                    "AAACGGTA".to_string()
                ]
            )
        );
        assert_eq!(
            3,
            Solution::min_mutation(
                "AAAAACCC".to_string(),
                "AACCCCCC".to_string(),
                vec![
                    "AAAACCCC".to_string(),
                    "AAACCCCC".to_string(),
                    "AACCCCCC".to_string()
                ]
            )
        );
    }
}
