use std::collections::HashMap;

struct Entry {
    timestamp: i32,
    value: String,
}

struct TimeMap {
    entries: HashMap<String, Vec<Entry>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            entries: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.entries
            .entry(key)
            .and_modify(|e| {
                e.push(Entry {
                    timestamp,
                    value: value.clone(),
                })
            })
            .or_insert(vec![Entry { timestamp, value }]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.entries.get(&key) {
            Some(entries) => {
                if entries.is_empty() {
                    return "".to_string();
                }

                let mut entry = &Entry {
                    timestamp: 0,
                    value: "".to_string(),
                };

                let mut left = 0;
                let mut right = entries.len() - 1;

                while left <= right && left < entries.len() && right < entries.len() {
                    let mid = left + (right - left) / 2;
                    let e = entries.get(mid).unwrap();

                    if e.timestamp == timestamp {
                        return e.value.clone();
                    } else if e.timestamp > timestamp {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                        entry = e;
                    }
                }

                entry.value.clone()
            }
            None => "".to_string(),
        }
    }
}
