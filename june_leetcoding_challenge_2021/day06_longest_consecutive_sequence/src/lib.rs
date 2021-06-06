pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let n = nums.len();
        let min = nums.iter().min().unwrap();
        let mut bucket = vec![vec![]; 10];

        for x in &nums {
            let num = (x - min) as usize;
            bucket[num % 10].push((num / 10, num));
        }

        while bucket[0].len() != n {
            let mut new_bucket = vec![vec![]; 10];
            for i in 0..10 {
                for j in 0..bucket[i].len() {
                    new_bucket[bucket[i][j].0 % 10].push((bucket[i][j].0 / 10, bucket[i][j].1));
                }
            }
            bucket = new_bucket;
        }

        let mut max = 0;
        let mut count = 1;
        for i in 1..n {
            if bucket[0][i].1 == bucket[0][i - 1].1 {
                continue;
            } else if bucket[0][i].1 == bucket[0][i - 1].1 + 1 {
                count += 1;
            } else {
                max = std::cmp::max(max, count);
                count = 1;
            }
        }

        std::cmp::max(max, count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
