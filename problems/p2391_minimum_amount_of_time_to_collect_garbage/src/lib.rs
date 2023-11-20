pub struct Solution {}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut travel_times = vec![0; garbage.len()];
        let mut truck_times = vec![0; 3];
        let mut truck_at = vec![0; 3];

        for i in 0..garbage.len() {
            if 0 < i {
                travel_times[i] = travel_times[i - 1] + travel[i - 1];
            }

            for g in garbage[i].as_bytes() {
                let j = match g {
                    b'M' => 0,
                    b'P' => 1,
                    b'G' => 2,
                    _ => unreachable!(),
                };
                if truck_at[j] != i {
                    truck_times[j] += travel_times[i] - travel_times[truck_at[j]];
                    truck_at[j] = i;
                }
                truck_times[j] += 1;
            }
        }

        truck_times.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2391() {
        assert_eq!(
            Solution::garbage_collection(
                vec![
                    "G".to_string(),
                    "P".to_string(),
                    "GP".to_string(),
                    "GG".to_string()
                ],
                vec![2, 4, 3]
            ),
            21
        );
        assert_eq!(
            Solution::garbage_collection(
                vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
                vec![3, 10]
            ),
            37
        );
    }
}
