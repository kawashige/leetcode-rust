pub struct Solution {}

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut visited = vec![0; next_visit.len()];
        let mut day = 2;

        for i in 1..next_visit.len() {
            visited[i] = day % M;
            day += 2 + (M + visited[i] - visited[next_visit[i] as usize]) % M;
            day %= M;
        }

        *visited.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1997() {
        assert_eq!(
            Solution::first_day_been_in_all_rooms(vec![
                0, 0, 1, 2, 4, 0, 1, 6, 0, 0, 2, 3, 4, 3, 4, 11, 6, 0, 16, 14, 20, 16, 9, 9, 1, 8,
                8, 4, 14, 13, 5, 12, 8, 18, 27, 34, 36, 13, 10, 35, 13, 31, 13, 29, 2, 45, 17, 30,
                10, 18, 41, 14, 41, 22, 2, 4, 1, 15, 27, 35, 12, 10, 46, 25, 61, 8, 65, 57, 48, 61,
                8, 35, 2, 66, 47, 5, 54, 76, 73, 51, 13, 64, 15, 2
            ]),
            409272772
        );
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0]), 2);
        assert_eq!(
            Solution::first_day_been_in_all_rooms(vec![
                0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12,
                13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23,
                23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 33, 33,
                34, 34, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44,
                44, 45, 45, 46, 46, 47, 47, 48
            ]),
            86417750
        );
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 1, 2, 0]), 6);
    }
}
