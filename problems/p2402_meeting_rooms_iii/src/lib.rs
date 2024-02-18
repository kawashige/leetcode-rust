pub struct Solution {}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable();

        let mut rooms = vec![0_usize; n as usize];
        let mut count = vec![0; n as usize];

        for i in 0..meetings.len() {
            if let Some(r) = (0..rooms.len()).min_by_key(|j| {
                rooms[*j].max(meetings[i][0] as usize)
            }) {
                rooms[r] = rooms[r].max(meetings[i][0] as usize) + (meetings[i][1] - meetings[i][0]) as usize;
                count[r] += 1;
            }
        }

        (0..count.len()).rev().max_by_key(|i| count[*i]).unwrap() as i32

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2402() {
        assert_eq!(
            Solution::most_booked(
                4,
                vec![
                    vec![18, 19],
                    vec![3, 12],
                    vec![17, 19],
                    vec![2, 13],
                ]
            ),
            0
        );
        assert_eq!(
            Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
            0
        );
        assert_eq!(
            Solution::most_booked(
                3,
                vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
            ),
            1
        );
    }
}
