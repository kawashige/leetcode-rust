pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        fn recurse(
            values: Vec<i32>,
            ops: Vec<char>,
            keys: Vec<String>,
            memo: &mut HashSet<String>,
        ) -> Vec<i32> {
            if ops.len() == 0 {
                return values;
            }
            let mut results = Vec::new();
            for i in 0..ops.len() {
                let mut new_keys = keys.clone();
                new_keys[i] = format!("({}{}{})", new_keys[i], ops[i], new_keys[i + 1]);
                new_keys.remove(i + 1);
                let mut new_ops = ops.clone();
                new_ops.remove(i);
                let new_key = (0..ops.len())
                    .map(|i| format!("{}{}", new_keys[i], ops[i]))
                    .chain(std::iter::once(new_keys.last().unwrap().clone()))
                    .collect::<String>();
                if memo.contains(&new_key) {
                    continue;
                }
                memo.insert(new_key);
                let mut new_values = values.clone();
                new_values[i] = match ops[i] {
                    '+' => values[i] + values[i + 1],
                    '-' => values[i] - values[i + 1],
                    '*' => values[i] * values[i + 1],
                    _ => unreachable!(),
                };
                new_values.remove(i + 1);
                results.append(&mut recurse(new_values, new_ops, new_keys, memo))
            }
            results
        }

        let mut ops = Vec::new();
        let mut values = Vec::new();
        let mut value = 0;
        for c in input.chars() {
            match c {
                '+' | '-' | '*' => {
                    values.push(value);
                    ops.push(c);
                    value = 0;
                }
                '0'..='9' => {
                    value = value * 10 + c.to_digit(10).unwrap() as i32;
                }
                _ => return Vec::new(),
            }
        }
        values.push(value);
        let keys = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        println!("{:?}, {:?}, {:?}", values, ops, keys);

        recurse(values, ops, keys, &mut HashSet::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0241() {
        assert_eq!(
            vec![0, 2],
            Solution::diff_ways_to_compute("2-1-1".to_string())
        );
        assert_eq!(
            vec![10, -14, -10, -10, -34],
            Solution::diff_ways_to_compute("2*3-4*5".to_string())
        );
        assert_eq!(
            vec![4, 4, 4, 4, 4],
            Solution::diff_ways_to_compute("1+1+1+1".to_string())
        );
    }
}
