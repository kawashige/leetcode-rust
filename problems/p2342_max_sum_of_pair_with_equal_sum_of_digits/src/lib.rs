pub struct Solution {}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut bucket = vec![vec![]; 100];
        for num in nums {
            let mut remains = num;
            let mut sum = 0;
            while 0 < remains {
                sum += remains % 10;
                remains /= 10;
            }
            bucket[sum as usize].push(num);
        }

        let mut result = -1;
        for i in 0..bucket.len() {
            if bucket[i].len() < 2 {
                continue;
            }
            bucket[i].sort_unstable();
            result = result.max(bucket[i][bucket[i].len() - 1] + bucket[i][bucket[i].len() - 2]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2342() {
        assert_eq!(Solution::maximum_sum(vec![18, 43, 36, 13, 7]), 54);
        assert_eq!(Solution::maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}
