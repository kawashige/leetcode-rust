pub struct Solution {}

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut y = 0;
        let mut stack: Vec<(i32, Vec<i32>)> = vec![(0, vec![-1])];
        let mut s = s.chars().collect::<Vec<_>>();

        for i in 0..s.len() {
            if s[i] == '0' {
                y -= 1;
                while !stack.is_empty() && y < stack.last().unwrap().0 {
                    let v = stack.pop().unwrap();
                    if 2 < v.1.len() {
                        let mut vals = Vec::new();
                        for j in 1..v.1.len() {
                            vals.push(
                                s[(v.1[j - 1] + 1) as usize..=v.1[j] as usize]
                                    .to_vec()
                                    .into_iter()
                                    .collect::<String>(),
                            );
                        }
                        vals.sort_unstable_by(|a, b| b.cmp(&a));
                        let tmp = vals.join("");
                        for j in 0..tmp.len() {
                            s[(v.1[0] + 1) as usize + j] = tmp.as_bytes()[j] as char;
                        }
                    }
                }
            } else {
                y += 1;
            }
            if !stack.is_empty() && stack.last().unwrap().0 == y {
                stack.last_mut().unwrap().1.push(i as i32);
            } else {
                stack.push((y, vec![i as i32]));
            }
        }

        while !stack.is_empty() {
            let v = stack.pop().unwrap();
            if 2 < v.1.len() {
                let mut vals = Vec::new();
                for j in 1..v.1.len() {
                    vals.push(
                        s[(v.1[j - 1] + 1) as usize..=v.1[j] as usize]
                            .to_vec()
                            .into_iter()
                            .collect::<String>(),
                    );
                }
                vals.sort_unstable_by(|a, b| b.cmp(&a));
                let tmp = vals.join("");
                for j in 0..tmp.len() {
                    s[(v.1[0] + 1) as usize + j] = tmp.as_bytes()[j] as char;
                }
            }
        }
        s.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0761() {
        assert_eq!(
            Solution::make_largest_special("1010101100".to_string()),
            "1100101010".to_string()
        );
        assert_eq!(
            Solution::make_largest_special("11011000".to_string()),
            "11100100".to_string()
        );
        assert_eq!(
            Solution::make_largest_special("10".to_string()),
            "10".to_string()
        );
    }
}
