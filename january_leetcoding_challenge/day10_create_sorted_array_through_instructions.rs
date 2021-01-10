pub struct Solution {}

impl Solution {
    pub fn merge(
        nums: &mut Vec<(i32, usize)>,
        counts: &mut Vec<Vec<i64>>,
        left: usize,
        mid: usize,
        right: usize,
    ) {
        let mut l = Vec::new();
        for i in 0..(mid - left) {
            l.push(nums[left + i]);
        }
        let mut r = Vec::new();
        for i in 0..(right - mid) {
            r.push(nums[mid + i]);
        }
        l.push((std::i32::MAX, 0));
        r.push((std::i32::MAX, 0));
        let mut i = 0;
        let mut j = 0;
        let mut same_count = 1;
        for k in left..right {
            if l[i].0 <= r[j].0 {
                nums[k] = l[i];
                if 0 < i && l[i - 1].0 == l[i].0 {
                    same_count += 1;
                } else {
                    same_count = 1;
                }
                i += 1;
            } else {
                nums[k] = r[j];
                counts[nums[k].1][0] += (i - if 0 < i && l[i - 1].0 == nums[k].0 {
                    same_count
                } else {
                    0
                }) as i64;
                counts[nums[k].1][1] += (l.len() - i - 1) as i64;
                j += 1;
            }
        }
    }

    pub fn merge_sort(
        nums: &mut Vec<(i32, usize)>,
        counts: &mut Vec<Vec<i64>>,
        left: usize,
        right: usize,
    ) {
        if left + 1 < right {
            let mid = (left + right) / 2;
            Self::merge_sort(nums, counts, left, mid);
            Self::merge_sort(nums, counts, mid, right);
            Self::merge(nums, counts, left, mid, right);
        }
    }

    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let l = instructions.len();
        let mut counts = vec![vec![0; 2]; l];
        let mut nums = instructions.into_iter().zip(0..).collect();
        Self::merge_sort(&mut nums, &mut counts, 0, l);
        (counts
            .into_iter()
            .map(|c| c.into_iter().min().unwrap())
            .sum::<i64>()
            % (1_000_000_000 + 7)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(0, Solution::create_sorted_array(vec![1]));
        assert_eq!(1, Solution::create_sorted_array(vec![1, 5, 6, 2]));
        assert_eq!(3, Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
        assert_eq!(
            4,
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2])
        );
        assert_eq!(
            118,
            Solution::create_sorted_array(vec![
                24, 17, 2, 7, 13, 12, 22, 21, 26, 6, 25, 15, 27, 11, 28, 32, 30, 33, 5, 23, 1, 29,
                20, 4, 31, 34, 16, 10, 9, 8, 3, 18, 14, 19
            ])
        );
    }
}
