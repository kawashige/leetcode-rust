pub struct Solution {}

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut sequences = vec![1, 2, 2];
        let mut count = 1;
        let mut next = 2;

        while sequences.len() < n as usize {
            let before = sequences.len() - 1;
            if sequences[next] == 1 {
                if sequences[before] == 1 {
                    sequences.push(2);
                } else {
                    sequences.push(1);
                    count += 1;
                }
            } else {
                if sequences[before] == 1 {
                    sequences.push(2);
                    sequences.push(2);
                } else {
                    sequences.push(1);
                    count += 1;
                    if sequences.len() < n as usize {
                        sequences.push(1);
                        count += 1;
                    }
                }
            }
            next += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0481() {
        assert_eq!(3, Solution::magical_string(6));
        assert_eq!(49972, Solution::magical_string(100000));
    }
}
