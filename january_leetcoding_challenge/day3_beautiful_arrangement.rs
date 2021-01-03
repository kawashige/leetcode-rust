pub struct Solution {}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut states = (1..=n).map(|i| vec![i]).collect::<Vec<Vec<i32>>>();
        let mut count = 0;
        while !states.is_empty() {
            let mut new_states = Vec::new();
            for s in states {
                for i in 1..=n {
                    let l = s.len() as i32 + 1;
                    if !s.contains(&i) && (l % i == 0 || i % l == 0) {
                        let mut new_s = s.clone();
                        new_s.push(i);
                        if new_s.len() == n as usize {
                            count += 1;
                        } else {
                            new_states.push(new_s);
                        }
                    }
                }
            }
            states = new_states;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day3() {
        assert_eq!(2, Solution::count_arrangement(2));
        assert_eq!(1, Solution::count_arrangement(1));
        assert_eq!(3, Solution::count_arrangement(3));
    }
}
