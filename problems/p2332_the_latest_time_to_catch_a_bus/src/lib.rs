pub struct Solution {}

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort_unstable();
        passengers.sort_unstable();

        let mut j = 0;
        let mut waiting = 0;

        for i in 0..buses.len() {
            while j < passengers.len() && passengers[j] <= buses[i] {
                j += 1;
                waiting += 1;
            }
            if i < buses.len() - 1 {
                waiting -= capacity.min(waiting);
            }
        }

        if waiting < capacity && (j == 0 || passengers[j - 1] != buses[buses.len() - 1]) {
            return buses[buses.len() - 1];
        }

        let mut i = j - (waiting - capacity.min(waiting)) as usize - 1;
        while 0 < i && passengers[i - 1] + 1 == passengers[i] {
            i -= 1;
        }

        passengers[i] - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2332() {
        assert_eq!(Solution::latest_time_catch_the_bus(vec![2], vec![2], 2), 1);
        assert_eq!(
            Solution::latest_time_catch_the_bus(vec![3], vec![2, 4], 2),
            3
        );
        assert_eq!(
            Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2),
            16
        );
        assert_eq!(
            Solution::latest_time_catch_the_bus(
                vec![20, 30, 10],
                vec![19, 13, 26, 4, 25, 11, 21],
                2
            ),
            20
        );
    }
}
