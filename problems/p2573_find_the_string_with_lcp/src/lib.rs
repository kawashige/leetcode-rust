pub struct Solution {}

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let mut chars = vec![-1; lcp.len()];
        let mut c = 0;

        for i in 0..lcp.len() {
            if chars[i] == -1 {
                if 26 <= c {
                    return Default::default();
                }
                chars[i] = c;
                c += 1;
            }
            for j in 0..lcp[i].len() {
                if i == j {
                    if lcp[i][j] != (lcp.len() - i) as i32 {
                        return Default::default();
                    }
                } else {
                    if (lcp[i][j] == 0) ^ (lcp[j][i] == 0) {
                        return Default::default();
                    }
                    if 1 < lcp[i][j]
                        && (lcp.len() <= i + 1
                            || lcp.len() <= j + 1
                            || lcp[i][j] != lcp[i + 1][j + 1] + 1)
                    {
                        return Default::default();
                    }

                    if 0 < lcp[i][j] {
                        chars[j] = chars[i];
                    }
                }
            }
        }

        let mut lcp2 = vec![vec![0; lcp.len()]; lcp.len()];
        for i in (0..lcp2.len()).rev() {
            for j in (0..=i).rev() {
                if i == j {
                    lcp2[i][j] = (lcp2.len() - i) as i32;
                } else if chars[i] == chars[j] {
                    lcp2[i][j] = if i + 1 == lcp2.len() {
                        0
                    } else {
                        lcp2[i + 1][j + 1]
                    } + 1;
                }
                if lcp[i][j] != lcp2[i][j] || lcp[j][i] != lcp2[i][j] {
                    return Default::default();
                }
            }
        }

        chars
            .into_iter()
            .map(|i| (b'a' + i as u8) as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2573() {
        assert_eq!(
            Solution::find_the_string(vec![
                vec![4, 0, 2, 0],
                vec![0, 3, 0, 1],
                vec![2, 0, 2, 0],
                vec![0, 1, 0, 1]
            ]),
            "abab".to_string()
        );
        assert_eq!(
            Solution::find_the_string(vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 1]
            ]),
            "aaaa".to_string()
        );
        assert_eq!(
            Solution::find_the_string(vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 3]
            ]),
            "".to_string()
        );
    }
}
