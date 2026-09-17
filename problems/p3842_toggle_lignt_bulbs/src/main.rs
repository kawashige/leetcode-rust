pub struct Solution {}

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut is_on = vec![false; 101];
        for b in bulbs {
            is_on[b as usize] = !is_on[b as usize];
        }
        (0..is_on.len() as i32)
            .filter(|b| is_on[*b as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3842() {
        assert_eq!(
            Solution::toggle_light_bulbs(vec![10, 30, 20, 10]),
            vec![20, 30]
        );
        assert_eq!(Solution::toggle_light_bulbs(vec![100, 100]), vec![]);
    }
}

fn main() {}
