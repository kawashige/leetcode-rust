use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    checked_in: HashMap<i32, (String, i32)>,
    travels: HashMap<(String, String), (i64, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checked_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((from_statin_name, from_t)) = self.checked_in.remove(&id) {
            let travel = self
                .travels
                .entry((from_statin_name, station_name))
                .or_insert((0, 0));
            travel.0 += (t - from_t) as i64;
            travel.1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(time, count)) = self.travels.get(&(start_station, end_station)) {
            return time as f64 / count as f64;
        }
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day20() {
        let mut obj = UndergroundSystem::new();
        obj.check_in(45, "Layton".to_string(), 3);
        obj.check_in(32, "Paradise".to_string(), 8);
        obj.check_in(27, "Layton".to_string(), 10);
        obj.check_out(45, "Waterloo".to_string(), 15);
        obj.check_out(27, "Waterloo".to_string(), 20);
        obj.check_out(32, "Cambridge".to_string(), 22);
        assert_eq!(
            obj.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.00000
        );
        assert_eq!(
            obj.get_average_time("Layton".to_string(), "Waterloo".to_string()),
            11.00000
        );
        obj.check_in(10, "Layton".to_string(), 24);
        assert_eq!(
            obj.get_average_time("Layton".to_string(), "Waterloo".to_string()),
            11.00000
        );
        obj.check_out(10, "Waterloo".to_string(), 38);
        assert_eq!(
            obj.get_average_time("Layton".to_string(), "Waterloo".to_string()),
            12.00000
        );
    }
}
