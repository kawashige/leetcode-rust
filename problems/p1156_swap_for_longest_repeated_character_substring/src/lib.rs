pub struct Solution {}

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut count = vec![0; 26];
        let mut prev = text.as_bytes()[0];
        let mut len = 1;
        let mut run_length = Vec::new();
        for i in 1..text.len() {
            if text.as_bytes()[i] != prev {
                run_length.push((prev, len));
                len = 1;
                count[(prev - b'a') as usize] += 1;
                prev = text.as_bytes()[i];
            } else {
                len += 1;
            }
        }
        run_length.push((prev, len));
        count[(prev - b'a') as usize] += 1;

        let mut result = 0;
        for i in 0..run_length.len() {
            result = result.max(
                run_length[i].1
                    + if 1 < run_length.len() && 1 < count[(run_length[i].0 - b'a') as usize] {
                        1
                    } else {
                        0
                    },
            );
            if 1 < i && run_length[i].0 == run_length[i - 2].0 && run_length[i - 1].1 == 1 {
                result = result.max(
                    run_length[i].1
                        + run_length[i - 2].1
                        + if 2 < count[(run_length[i].0 - b'a') as usize] {
                            1
                        } else {
                            0
                        },
                );
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1156() {
        assert_eq!(Solution::max_rep_opt1("abcdef".to_string()), 1);
        assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
        assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
        assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
    }
}
