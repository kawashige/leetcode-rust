use std::collections::HashMap;

#[derive(Default)]
struct TweetCounts {
    tweets: HashMap<String, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {
    fn new() -> Self {
        Default::default()
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        if let Some(t) = self.tweets.get_mut(&tweet_name) {
            match t.binary_search(&time) {
                Ok(i) => t.insert(i, time),
                Err(i) => t.insert(i, time),
            }
        } else {
            self.tweets.insert(tweet_name, vec![time]);
        }
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let span = Self::time_span(&freq);
        let mut result = vec![0; ((end_time - start_time) / span) as usize + 1];

        if let Some(t) = self.tweets.get(&tweet_name) {
            let start_index = t.binary_search(&start_time).map_or_else(|e| e, |v| v);
            let end_index = t.binary_search(&end_time).map_or_else(|e| e, |v| v + 1);

            for i in start_index..end_index {
                result[((t[i] - start_time) / span) as usize] += 1;
            }
        }

        result
    }

    fn time_span(freq: &str) -> i32 {
        match freq {
            "minute" => 60,
            "hour" => 3600,
            "day" => 86400,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1348() {
        let mut obj = TweetCounts::new();
        obj.record_tweet("tweet3".to_string(), 0);
        obj.record_tweet("tweet3".to_string(), 60);
        obj.record_tweet("tweet3".to_string(), 10);
        assert_eq!(
            obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59),
            vec![2]
        );
        assert_eq!(
            obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60),
            vec![2, 1]
        );
        obj.record_tweet("tweet3".to_string(), 120);
        assert_eq!(
            obj.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210),
            vec![4]
        );
    }
}
