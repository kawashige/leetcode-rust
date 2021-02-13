use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut queues = vec![VecDeque::new(); 2];
        for (i, c) in senate.chars().enumerate() {
            queues[if c == 'R' { 0 } else { 1 }].push_back(i);
        }
        let mut prev = senate.len();
        while !queues[0].is_empty() && !queues[1].is_empty() {
            let mut current = (0..2).min_by_key(|i| queues[*i][0]).unwrap();
            if queues[current][0] <= prev && prev < queues[(current + 1) % 2][0] {
                current = (current + 1) % 2;
            }
            let q = queues[current].pop_front().unwrap();
            prev = q;
            queues[current].push_back(q);
            queues[(current + 1) % 2].pop_front();
        }

        if queues[0].is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0649() {
        assert_eq!(
            Solution::predict_party_victory("RRDDD".to_string()),
            "Radiant".to_string()
        );
        assert_eq!(
            Solution::predict_party_victory("DDRRR".to_string()),
            "Dire".to_string()
        );
        assert_eq!(
            Solution::predict_party_victory("R".to_string()),
            "Radiant".to_string()
        );
        assert_eq!(
            Solution::predict_party_victory("RD".to_string()),
            "Radiant".to_string()
        );
        assert_eq!(
            Solution::predict_party_victory("RDD".to_string()),
            "Dire".to_string()
        );
    }
}
