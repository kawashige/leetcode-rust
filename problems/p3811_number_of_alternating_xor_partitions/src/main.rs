use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn alternating_xor(nums: Vec<i32>, target1: i32, target2: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut xor = 0;

        for i in 0..nums.len() {
            xor ^= nums[i];
            let mut c1 = if xor == target1 { 1 } else { 0 };
            c1 = (c1 + map2.get(&(xor ^ target1)).unwrap_or(&0)) % M;
            let c2 = *map1.get(&(xor ^ target2)).unwrap_or(&0);

            if i == nums.len() - 1 {
                return ((c1 + c2) % M) as i32;
            }

            let val1 = (map1.get(&xor).unwrap_or(&0) + c1) % M;
            map1.insert(xor, val1);
            let val2 = (map2.get(&xor).unwrap_or(&0) + c2) % M;
            map2.insert(xor, val2);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3811() {
        assert_eq!(Solution::alternating_xor(vec![2, 3, 1, 4], 1, 5), 1);
        assert_eq!(Solution::alternating_xor(vec![1, 0, 0], 1, 0), 3);
        assert_eq!(Solution::alternating_xor(vec![7], 1, 7), 0);
    }
}

fn main() {}
