pub struct Solution {}

impl Solution {
    pub fn count_pairs(max: i32, sum: i32, both: i32) -> i32 {
        if max == 0 {
            return 0;
        }
        ((sum + both) / 2).min(sum + both - max.max(both))
    }

    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let mut x_at_first = [0; 10];
        let mut x_at_second = [0; 10];
        let mut x_at_both = 0;
        let char_index = (x as u8 - b'a') as usize;

        for i in 0..cards.len() {
            let first = (cards[i].as_bytes()[0] - b'a') as usize;
            let second = (cards[i].as_bytes()[1] - b'a') as usize;
            if first == char_index && second == char_index {
                x_at_both += 1;
            } else if first == char_index {
                x_at_first[second] += 1;
            } else if second == char_index {
                x_at_second[first] += 1;
            }
        }

        let first_max = *x_at_first.iter().max().unwrap();
        let first_sum = x_at_first.iter().sum::<i32>();
        let second_max = *x_at_second.iter().max().unwrap();
        let second_sum = x_at_second.iter().sum::<i32>();

        let mut result = 0;
        for i in 0..=x_at_both {
            result = result.max(
                Self::count_pairs(first_max, first_sum, i)
                    + Self::count_pairs(second_max, second_sum, x_at_both - i),
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3664() {
        assert_eq!(
            Solution::score(
                vec![
                    "aa".to_string(),
                    "ab".to_string(),
                    "ba".to_string(),
                    "ac".to_string()
                ],
                'a'
            ),
            2
        );
        assert_eq!(
            Solution::score(
                vec!["aa".to_string(), "ab".to_string(), "ba".to_string()],
                'a'
            ),
            1
        );
        assert_eq!(
            Solution::score(
                vec![
                    "aa".to_string(),
                    "ab".to_string(),
                    "ba".to_string(),
                    "ac".to_string()
                ],
                'b'
            ),
            0
        );
    }
}
