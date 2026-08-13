impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let res = -1;

        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high && low < nums.len() && high < nums.len() {
            let mid = (low + high) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        res
    }
}
