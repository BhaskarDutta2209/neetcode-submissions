impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let mut high = heights.len() - 1;
        let mut low = 0;

        while low < high {
            let h = if heights[high] > heights[low] {
                heights[low]
            } else {
                heights[high]
            };
            let area = h * (high - low) as i32;
            if area > max_area {
                max_area = area;
            }

            if heights[high] > heights[low] {
                low += 1;
            } else {
                high -= 1;
            }
        }

        max_area
    }
}
