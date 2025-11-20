pub struct Solution {}

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
        let mut used = vec![vec![]; intervals.len()];

        let mut size = 0;

        for i in 0..intervals.len() {
            if 1 < used[i].len() {
                continue;
            }
            let mut nums = Vec::new();
            for num in (intervals[i][0]..=intervals[i][1]).rev() {
                if !used[i].is_empty() && used[i][0] == num {
                    continue;
                }
                nums.push(num);
                used[i].push(num);
                size += 1;
                if used[i].len() == 2 {
                    break;
                }
            }
            for j in i + 1..intervals.len() {
                for k in (0..nums.len()).rev() {
                    if (intervals[j][0]..=intervals[j][1]).contains(&nums[k]) {
                        used[j].push(nums[k]);
                    }
                }
            }
        }

        size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0757() {
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 3], vec![3, 7], vec![8, 9]]),
            5
        );
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
            3
        );
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
            5
        );
    }
}
