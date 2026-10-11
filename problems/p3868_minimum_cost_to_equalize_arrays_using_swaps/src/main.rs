use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn min_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        for n in nums1 {
            (*map.entry(n).or_insert([0; 2]))[0] += 1;
        }
        for n in nums2 {
            (*map.entry(n).or_insert([0; 2]))[1] += 1;
        }

        let mut move_count = [0; 2];

        for (_, count) in map {
            if count[1] < count[0] {
                if (count[0] - count[1]) % 2 != 0 {
                    return -1;
                }
                move_count[0] += (count[0] - count[1]) / 2;
            } else if count[0] < count[1] {
                if (count[1] - count[0]) % 2 != 0 {
                    return -1;
                }
                move_count[1] += (count[1] - count[0]) / 2;
            }
        }

        if move_count[0] != move_count[1] {
            -1
        } else {
            move_count[0]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3868() {
        assert_eq!(Solution::min_cost(vec![10, 20], vec![20, 10]), 0);
        assert_eq!(Solution::min_cost(vec![10, 10], vec![20, 20]), 1);
        assert_eq!(Solution::min_cost(vec![10, 20], vec![30, 40]), -1);
    }
}

fn main() {}
