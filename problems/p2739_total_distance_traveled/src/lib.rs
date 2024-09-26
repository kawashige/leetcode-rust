pub struct Solution {}

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let mut main_tank = main_tank;
        let mut additional_tank = additional_tank;
        let mut result = 0;

        while 0 < main_tank {
            result += 1;
            main_tank -= 1;
            if result % 5 == 0 && 0 < additional_tank {
                main_tank += 1;
                additional_tank -= 1;
            }
        }

        result * 10
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2739() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
        assert_eq!(Solution::distance_traveled(1, 2), 10);
    }
}
