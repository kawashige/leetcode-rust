pub struct Solution {}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        logs.into_iter()
            .fold(
                (vec![0; n as usize], vec![]),
                |(mut time, mut stack), log| {
                    let (pid, log_type, timestamp) = {
                        let mut ls = log.split(":");
                        (
                            ls.next().unwrap().parse::<usize>().unwrap(),
                            ls.next().unwrap(),
                            ls.next().unwrap().parse::<i32>().unwrap(),
                        )
                    };
                    if log_type == "start" {
                        stack.push((timestamp, 0));
                    } else {
                        let start = stack.pop().unwrap();
                        let elapsed = timestamp - start.0 - start.1 + 1;
                        time[pid] += elapsed;
                        if !stack.is_empty() {
                            stack.last_mut().unwrap().1 += elapsed + start.1;
                        }
                    }
                    (time, stack)
                },
            )
            .0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0636() {
        assert_eq!(
            Solution::exclusive_time(
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:1".to_string(),
                    "0:start:2".to_string(),
                    "0:end:3".to_string(),
                    "0:end:4".to_string(),
                    "0:end:5".to_string()
                ]
            ),
            vec![6]
        );
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string()
                ]
            ),
            vec![3, 4]
        );
        assert_eq!(
            Solution::exclusive_time(
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "0:start:6".to_string(),
                    "0:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![7, 1]
        );
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:7".to_string(),
                    "1:end:7".to_string(),
                    "0:end:8".to_string()
                ]
            ),
            vec![8, 1]
        );
        assert_eq!(
            Solution::exclusive_time(1, vec!["0:start:0".to_string(), "0:end:0".to_string()]),
            vec![1]
        )
    }
}
