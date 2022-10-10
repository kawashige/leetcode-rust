pub struct Solution {}

impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut acc = vec![0; nums.len()];
        acc[0] = nums[0];
        for i in 1..nums.len() {
            acc[i] = acc[i - 1] + nums[i];
        }

        let mut count = 0;

        for r in 2..nums.len() {
            let right = acc[acc.len() - 1] - acc[r - 1];
            let left_and_mid = acc[r - 1];

            // find right most
            if left_and_mid - acc[0] < acc[0] {
                continue;
            }

            let mut ok = 0;
            let mut ng = r - 1;

            while ok + 1 < ng {
                let mid = (ok + ng) / 2;
                if acc[mid] <= left_and_mid - acc[mid] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            let right_most = ok;

            // find left most
            let mut left_most = 0;
            if right < left_and_mid - acc[0] {
                let mut ng = 0;
                let mut ok = r - 1;

                while ng + 1 < ok {
                    let mid = (ng + ok) / 2;
                    if left_and_mid - acc[mid] <= right {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }

                left_most = ok;
            }

            if left_most <= right_most {
                count = (count + right_most - left_most + 1) % M;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1712() {
        // assert_eq!(Solution::ways_to_split(vec![1, 1, 1]), 1);
        assert_eq!(Solution::ways_to_split(vec![1, 2, 2, 2, 5, 0]), 3);
        assert_eq!(Solution::ways_to_split(vec![3, 2, 1]), 0);
    }
}
