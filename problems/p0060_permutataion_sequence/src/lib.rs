pub struct Solution {}

impl Solution {
    pub fn recurse(k: i32, seen: &mut Vec<bool>, result: &mut String) {
        if k == 0 {
            return;
        }

        let c = (1..seen.len()).filter(|i| !seen[*i]).count() as i32;
        let d = (1..c).fold(1, |acc, d| acc * d);

        let mut k = k;
        for i in 1..seen.len() {
            if seen[i] {
                continue;
            }
            if k <= d {
                println!("i: {}, k: {}, d: {}", i, k, d);
                seen[i] = true;
                (*result).push((b'0' + i as u8) as char);
                Self::recurse(k, seen, result);
                return;
            }
            k -= d;
        }
    }

    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut result = String::new();
        Self::recurse(k, &mut vec![false; n as usize + 1], &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0060() {
        assert_eq!(Solution::get_permutation(9, 362880), "213".to_string());
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_string());
    }
}
