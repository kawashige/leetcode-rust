pub struct Solution {}

use std::collections::BTreeMap;

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let n = s.len();
        let mut s = s.into_bytes();
        let qc = query_characters.as_bytes();
        let mut segs: BTreeMap<usize, usize> = BTreeMap::new();
        let mut lens: BTreeMap<i32, i32> = BTreeMap::new();

        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            segs.insert(i, j - 1);
            *lens.entry((j - i) as i32).or_insert(0) += 1;
            i = j;
        }

        let k = query_indices.len();
        let mut ans = vec![0; k];

        for q in 0..k {
            let pos = query_indices[q] as usize;
            let ch = qc[q];

            if s[pos] != ch {
                let (&L, &R) = segs.range(..=pos).next_back().unwrap();
                segs.remove(&L);
                let old_len = (R - L + 1) as i32;
                *lens.get_mut(&old_len).unwrap() -= 1;
                if lens[&old_len] == 0 {
                    lens.remove(&old_len);
                }

                if L <= pos - 1 {
                    segs.insert(L, pos - 1);
                    *lens.entry((pos - L) as i32).or_insert(0) += 1;
                }
                if pos + 1 <= R {
                    segs.insert(pos + 1, R);
                    *lens.entry((R - pos) as i32).or_insert(0) += 1;
                }

                let mut new_l = pos;
                let mut new_r = pos;

                if pos + 1 < n && s[pos + 1] == ch {
                    if let Some(&right_r) = segs.get(&(pos + 1)) {
                        let right_len = (right_r - (pos + 1) + 1) as i32;
                        *lens.get_mut(&right_len).unwrap() -= 1;
                        if lens[&right_len] == 0 {
                            lens.remove(&right_len);
                        }
                        new_r = right_r;
                        segs.remove(&(pos + 1));
                    }
                }

                if pos > 0 && s[pos - 1] == ch {
                    if let Some((&left_l, &left_r)) = segs.range(..pos).next_back() {
                        if left_r == pos - 1 {
                            let left_len = (left_r - left_l + 1) as i32;
                            *lens.get_mut(&left_len).unwrap() -= 1;
                            if lens[&left_len] == 0 {
                                lens.remove(&left_len);
                            }
                            new_l = left_l;
                            segs.remove(&left_l);
                        }
                    }
                }

                segs.insert(new_l, new_r);
                *lens.entry((new_r - new_l + 1) as i32).or_insert(0) += 1;
                s[pos] = ch;
            }

            ans[q] = *lens.keys().next_back().unwrap();
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2213() {
        assert_eq!(
            Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3]),
            vec![3, 3, 4]
        );
        assert_eq!(
            Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1]),
            vec![2, 3]
        );
    }
}

fn main() {}
