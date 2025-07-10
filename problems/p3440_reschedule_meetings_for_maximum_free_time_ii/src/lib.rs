pub struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut gaps = Vec::new();
        gaps.push(start_time[0]);
        gaps.push(event_time - end_time[end_time.len() - 1]);
        for i in 1..start_time.len() {
            gaps.push(start_time[i] - end_time[i - 1]);
        }
        gaps.sort_unstable_by(|a, b| b.cmp(&a));

        let mut result = std::i32::MIN;

        for i in 0..start_time.len() {
            let mut gap = vec![
                start_time[i] - if i == 0 { 0 } else { end_time[i - 1] },
                if i == start_time.len() - 1 {
                    event_time
                } else {
                    start_time[i + 1]
                } - end_time[i],
            ];
            gap.sort_unstable_by(|a, b| b.cmp(&a));
            let mut k = 0;
            let mut is_ok = false;
            for j in 0..3 {
                if k < gap.len() && gaps[j] == gap[k] {
                    k += 1;
                    continue;
                }
                if end_time[i] - start_time[i] <= gaps[j] {
                    is_ok = true;
                    break;
                }
            }
            result = result.max(
                gap[0]
                    + gap[1]
                    + if is_ok {
                        end_time[i] - start_time[i]
                    } else {
                        0
                    },
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3440() {
        assert_eq!(Solution::max_free_time(5, vec![1, 3], vec![2, 5]), 2);
        assert_eq!(
            Solution::max_free_time(10, vec![0, 7, 9], vec![1, 8, 10]),
            7
        );
        assert_eq!(
            Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10]),
            6
        );
    }
}
