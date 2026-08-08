struct Car {
    position: i32,
    speed: i32,
    time: f32,
}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut merged_array = Vec::new();
        let mut stack = Vec::new();

        for (index, _) in position.iter().enumerate() {
            merged_array.push(Car {
                position: position[index],
                speed: speed[index],
                time: (target - position[index]) as f32 / speed[index] as f32,
            });
        }

        merged_array.sort_by(|a, b| b.position.cmp(&a.position));

        for entry in merged_array {
            if stack.is_empty() {
                stack.push(entry);
            } else {
                if stack.last().unwrap().time < entry.time {
                    stack.push(entry);
                }
            }
        }

        stack.len() as i32
    }
}

