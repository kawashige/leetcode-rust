use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut nums = vec![-1; 32];
        for i in 0..arr.len() {
            if arr[i] == 0 {
                set.insert(0);
                continue;
            }
            for j in 0..32 {
                if arr[i] & 1 << j > 0 {
                    nums[j] = i as i32;
                }
            }

            let mut tmp = nums
                .clone()
                .into_iter()
                .enumerate()
                .collect::<Vec<(usize, i32)>>();
            tmp.sort_unstable_by(|a, b| b.1.cmp(&a.1));

            let mut k = 0;
            let mut x = 0;
            while k < 32 {
                if tmp[k].1 < 0 {
                    break;
                }
                x |= 1 << tmp[k].0;
                while k + 1 < 32 && tmp[k].1 == tmp[k + 1].1 {
                    k += 1;
                    x |= 1 << tmp[k].0;
                }

                set.insert(x);
                k += 1;
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0898() {
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![3, 11]), 2);
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
    }
}
