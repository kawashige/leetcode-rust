pub struct Solution {}

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_by_key(|num| {
            if num == &0 {
                return mapping[0];
            }
            let mut key = 0;
            let mut remains = *num;
            let mut r = 1;
            while 0 < remains {
                key += r * mapping[(remains % 10) as usize];
                remains /= 10;
                r *= 10;
            }
            key
        });
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2191() {
        assert_eq!(
            Solution::sort_jumbled(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            ),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        assert_eq!(
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        );
        assert_eq!(
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
            vec![123, 456, 789]
        );
    }
}
