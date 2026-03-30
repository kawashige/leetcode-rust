pub struct Solution {}

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut b_min1 = vec![std::i32::MAX; n + 1];
        let mut b_min2 = vec![std::i32::MAX; n + 1];
        for pair in conflicting_pairs {
            let a = pair[0].min(pair[1]) as usize;
            let b = pair[0].max(pair[1]);
            if b < b_min1[a] {
                b_min2[a] = b_min1[a];
                b_min1[a] = b;
            } else if b < b_min2[a] {
                b_min2[a] = b;
            }
        }

        let mut result = 0;
        let mut ib1 = n;
        let mut b2 = std::i32::MAX;
        let mut del_count = vec![0; n + 1];
        for i in (1..=n).rev() {
            if b_min1[i] < b_min1[ib1] {
                b2 = b2.min(b_min1[ib1]);
                ib1 = i;
            } else {
                b2 = b2.min(b_min1[i]);
            }
            result += (b_min1[ib1].min(n as i32 + 1) - i as i32) as i64;
            del_count[ib1] +=
                (b2.min(b_min2[ib1]).min(n as i32 + 1) - b_min1[ib1].min(n as i32 + 1)) as i64
        }

        result + del_count.iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3480() {
        assert_eq!(Solution::max_subarrays(4, vec![vec![2, 3], vec![1, 4]]), 9);
        assert_eq!(
            Solution::max_subarrays(5, vec![vec![1, 2], vec![2, 5], vec![3, 5]]),
            12
        );
    }
}
