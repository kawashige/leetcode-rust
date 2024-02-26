pub struct Solution {}

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        players.sort_unstable();
        trainers.sort_unstable();

        let mut j = 0;
        let mut count = 0;

        for i in 0..players.len() {
            while j < trainers.len() && trainers[j] < players[i] {
                j += 1;
            }
            if trainers.len() <= j {
                break;
            }
            count += 1;
            j += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2410() {
        assert_eq!(
            Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]),
            2
        );
        assert_eq!(
            Solution::match_players_and_trainers(vec![1, 1, 1], vec![10]),
            1
        );
    }
}
