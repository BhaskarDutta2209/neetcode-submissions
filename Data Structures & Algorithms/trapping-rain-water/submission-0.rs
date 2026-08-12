impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_water = 0;
        let mut left_pointer = 0;
        let mut right_pointer = height.len() - 1;
        let mut max_left = height[left_pointer];
        let mut max_right = height[right_pointer];

        while left_pointer < right_pointer {
            let max_height = if max_left > max_right {
                max_right
            } else {
                max_left
            };

            if max_left < max_right {
                let item = height[left_pointer+1];
                let water = max_height - item;
                if water > 0 {
                    max_water += water;
                }
                if item > max_left {
                    max_left = item;
                }
                left_pointer += 1;
            } else {
                let item = height[right_pointer-1];
                let water = max_height - item;
                if water > 0 {
                    max_water += water;
                }
                if item > max_right {
                    max_right = item;
                }
                right_pointer -= 1;
            }
        }

        max_water
    }
}
