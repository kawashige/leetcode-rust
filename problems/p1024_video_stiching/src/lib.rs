pub struct Solution {}

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, time: i32) -> i32 {
        clips.sort_unstable_by_key(|v| -v[0]);

        let mut count = 0;
        let mut t = 0;
        let mut next_t = 0;

        while t < time && !clips.is_empty() && clips.last().unwrap()[0] <= t {
            while !clips.is_empty() && clips.last().unwrap()[0] <= t {
                next_t = next_t.max(clips.pop().unwrap()[1]);
            }
            t = next_t;
            count += 1;
        }

        if t < time {
            -1
        } else {
            count
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1024() {
        assert_eq!(
            Solution::video_stitching(
                vec![
                    vec![0, 2],
                    vec![4, 6],
                    vec![8, 10],
                    vec![1, 9],
                    vec![1, 5],
                    vec![5, 9]
                ],
                10
            ),
            3
        );
        assert_eq!(
            Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5),
            -1
        );
        assert_eq!(
            Solution::video_stitching(
                vec![
                    vec![0, 1],
                    vec![6, 8],
                    vec![0, 2],
                    vec![5, 6],
                    vec![0, 4],
                    vec![0, 3],
                    vec![6, 7],
                    vec![1, 3],
                    vec![4, 7],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 7],
                    vec![6, 9]
                ],
                9
            ),
            3
        );
        assert_eq!(
            Solution::video_stitching(vec![vec![0, 4], vec![2, 8]], 5),
            2
        );
    }
}
