pub struct Solution {}

impl Solution {
    pub fn count_cost(start_at: i32, move_cost: i32, push_cost: i32, target: &[i32]) -> i32 {
        let mut current = start_at;
        let mut cost = push_cost * target.len() as i32;

        for t in target.iter() {
            if t != &current {
                current = *t;
                cost += move_cost;
            }
        }

        cost
    }

    pub fn to_digits(minutes: i32, seconds: i32) -> Vec<i32> {
        format!("{}{:0>2}", minutes, seconds)
            .trim_start_matches('0')
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }

    pub fn min_cost_set_time(
        start_at: i32,
        move_cost: i32,
        push_cost: i32,
        target_seconds: i32,
    ) -> i32 {
        let mut minutes = target_seconds / 60;
        let mut seconds = target_seconds % 60;
        if 99 < minutes {
            minutes -= 1;
            seconds += 60;
        }

        Self::count_cost(
            start_at,
            move_cost,
            push_cost,
            &Self::to_digits(minutes, seconds),
        )
        .min(if 0 < minutes && seconds < 40 {
            Self::count_cost(
                start_at,
                move_cost,
                push_cost,
                &Self::to_digits(minutes - 1, seconds + 60),
            )
        } else {
            std::i32::MAX
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2162() {
        assert_eq!(Solution::min_cost_set_time(1, 2, 1, 600), 6);
        assert_eq!(Solution::min_cost_set_time(0, 1, 2, 76), 6);
    }
}
