struct StackElement {
    index: i32,
    height: i32,
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack = Vec::new();

        for (index, height) in heights.iter().enumerate() {
            if stack.is_empty() {
                stack.push(StackElement {
                    index: index as i32,
                    height: *height,
                });
                max_area = *height;
                continue;
            }

            let mut new_entry_index = index as i32;
            while !stack.is_empty() && stack.last().unwrap().height > *height {
                let entry = stack.pop().unwrap();
                let area = (index as i32 - entry.index) * entry.height;
                if area > max_area {
                    max_area = area;
                }
                new_entry_index = entry.index;
            }

            stack.push(StackElement {
                index: new_entry_index,
                height: *height,
            });
        }

        while let Some(entry) = stack.pop() {
            let area = (heights.len() as i32 - entry.index) * entry.height;
            if area > max_area {
                max_area = area;
            }
        }

        max_area
    }
}
