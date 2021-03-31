pub struct Solution {}

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let mut target = target.chars().collect::<Vec<char>>();
        let stamp = stamp.chars().collect::<Vec<char>>();

        let mut changed = 0;
        let mut used = vec![false; target.len()];
        let mut result = Vec::new();

        while changed < target.len() {
            let tmp = changed;
            for i in 0..(target.len() - stamp.len() + 1) {
                if changed < target.len()
                    && !used[i]
                    && (0..stamp.len()).all(|j| target[i + j] == '?' || target[i + j] == stamp[j])
                {
                    for j in 0..stamp.len() {
                        if target[i + j] != '?' {
                            changed += 1;
                            target[i + j] = '?';
                        }
                    }
                    result.push(i as i32);
                    used[i] = true;
                }
            }
            if tmp == changed {
                break;
            }
        }

        if target.iter().all(|c| c == &'?') {
            result.into_iter().rev().collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day31() {
        assert_eq!(
            Solution::moves_to_stamp("abc".to_string(), "ababc".to_string()),
            vec![0, 2]
        );
        assert_eq!(
            Solution::moves_to_stamp("abca".to_string(), "aabcaca".to_string()),
            vec![3, 0, 1]
        );
    }
}
