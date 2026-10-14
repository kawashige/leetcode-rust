pub struct Solution {}

impl Solution {
    pub fn score_validator(events: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; 2];

        for e in events {
            match e.as_str() {
                "W" => {
                    result[1] += 1;
                    if result[1] == 10 {
                        return result;
                    }
                }
                "WD" | "NB" => {
                    result[0] += 1;
                }
                _ => {
                    result[0] += e.parse::<i32>().unwrap();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3921() {
        assert_eq!(
            Solution::score_validator(vec![
                "1".to_string(),
                "4".to_string(),
                "W".to_string(),
                "6".to_string(),
                "WD".to_string()
            ]),
            vec![12, 1]
        );
        assert_eq!(
            Solution::score_validator(vec![
                "WD".to_string(),
                "NB".to_string(),
                "0".to_string(),
                "4".to_string(),
                "4".to_string()
            ]),
            vec![10, 0]
        );
        assert_eq!(
            Solution::score_validator(vec![
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string(),
                "W".to_string()
            ]),
            vec![0, 10]
        );
    }
}

fn main() {}
