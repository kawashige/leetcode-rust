pub struct Solution {}

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut steps = 0;
        let mut remains = capacity;

        for i in 0..plants.len() {
            steps += 1;
            remains -= plants[i];

            if i + 1 < plants.len() && remains < plants[i + 1] {
                steps += 2 * (i + 1);
                remains = capacity
            }
        }

        steps as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2079() {
        assert_eq!(Solution::watering_plants(vec![2, 2, 3, 3,], 5), 14);
        assert_eq!(Solution::watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
        assert_eq!(Solution::watering_plants(vec![7, 7, 7, 7, 7, 7, 7], 8), 49);
    }
}
