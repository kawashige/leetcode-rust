pub struct Solution {}

impl Solution {
    pub fn to_i64(digits: &[i64]) -> i64 {
        digits.iter().rev().fold(0, |acc, d| acc * 10 + *d as i64)
    }

    pub fn nearest_palindromic(n: String) -> String {
        if n.len() == 1 {
            return (n.parse::<usize>().unwrap() - 1).to_string();
        }
        if n == "10".to_string() {
            return "9".to_string();
        }

        let org = n
            .as_bytes()
            .iter()
            .rev()
            .map(|b| (b - b'0') as i64)
            .collect::<Vec<_>>();
        let org_i64 = Self::to_i64(&org);

        let mut candidates = Vec::new();

        let mut cand1 = org.clone();
        let l = cand1.len();
        for i in 0..l / 2 {
            cand1[i] = cand1[l - 1 - i];
        }
        let cand1_i64 = Self::to_i64(&cand1);
        let d1 = (org_i64 - cand1_i64).abs();
        if org_i64 != cand1_i64 {
            candidates.push((d1, cand1_i64));
        }

        let mut cand2 = org.clone();
        let mut i = l / 2;
        cand2[i] += 1;
        while cand2[i] == 10 {
            cand2[i] = 0;
            if i + 1 < cand2.len() {
                cand2[i + 1] += 1;
            } else {
                cand2.push(1);
            }
            i += 1;
        }
        let l = cand2.len();
        for i in 0..l / 2 {
            cand2[i] = cand2[l - 1 - i];
        }
        let cand2_i64 = Self::to_i64(&cand2);
        let d2 = (org_i64 - Self::to_i64(&cand2)).abs();
        if org_i64 != cand2_i64 {
            candidates.push((d2, cand2_i64));
        }

        let mut cand3 = org.clone();
        let mut i = l / 2;
        cand3[i] -= 1;
        while cand3[i] == -1 {
            cand3[i] = 9;
            cand3[i + 1] -= 1;
            i += 1;
        }
        while cand3.last() == Some(&0) {
            cand3.pop();
        }
        let l = cand3.len();
        if org.len() != l && l % 2 == 1 {
            cand3[l / 2] = 9;
        }
        for i in 0..l / 2 {
            cand3[i] = cand3[l - 1 - i];
        }
        let cand3_i64 = Self::to_i64(&cand3);
        let d3 = (org_i64 - Self::to_i64(&cand3)).abs();
        if org_i64 != cand3_i64 {
            candidates.push((d3, cand3_i64));
        }

        candidates.sort_unstable();
        candidates[0].1.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0564() {
        assert_eq!(
            Solution::nearest_palindromic("11911".to_string()),
            "11811".to_string()
        );
        assert_eq!(
            Solution::nearest_palindromic("10".to_string()),
            "9".to_string()
        );
        assert_eq!(
            Solution::nearest_palindromic("1000".to_string()),
            "999".to_string()
        );
        assert_eq!(
            Solution::nearest_palindromic("999".to_string()),
            "1001".to_string()
        );
        assert_eq!(
            Solution::nearest_palindromic("123".to_string()),
            "121".to_string()
        );
        assert_eq!(
            Solution::nearest_palindromic("1".to_string()),
            "0".to_string()
        );
    }
}
