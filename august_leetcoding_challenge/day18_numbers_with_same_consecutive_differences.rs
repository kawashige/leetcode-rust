pub struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        fn find(n: i32, k: i32, seq: Vec<i32>) -> Vec<Vec<i32>> {
            if seq.len() as i32 == n {
                return vec![seq];
            }
            let mut result = Vec::new();
            let c1 = k + seq[seq.len() - 1];
            if -1 < c1 && c1 < 10 {
                let mut new_seq = seq.clone();
                new_seq.push(c1);
                result.append(&mut find(n, k, new_seq));
            }
            let c2 = seq[seq.len() - 1] - k;
            if -1 < c2 && c2 < 10 && c1 != c2 {
                let mut new_seq = seq.clone();
                new_seq.push(c2);
                result.append(&mut find(n, k, new_seq));
            }
            result
        }

        let mut result = Vec::new();
        let start = match n {
            1 => 0,
            _ => 1,
        };
        for i in start..=9 {
            result.append(&mut find(n, k, vec![i]));
        }
        result
            .iter()
            .map(|v| {
                v.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .concat()
            })
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day18() {
        assert_eq!(
            vec![181, 292, 707, 818, 929],
            Solution::nums_same_consec_diff(3, 7)
        );
        let mut result = Solution::nums_same_consec_diff(2, 1);
        result.sort();
        assert_eq!(
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98],
            result
        );
        assert_eq!(
            vec![111, 222, 333, 444, 555, 666, 777, 888, 999],
            Solution::nums_same_consec_diff(3, 0)
        );

        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::nums_same_consec_diff(1, 3)
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::nums_same_consec_diff(1, 0)
        );
    }
}
