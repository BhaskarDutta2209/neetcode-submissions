impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut res = 100000001;

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right && left < nums.len() && right < nums.len() {
            if nums[left] < nums[right] {
                if nums[left] < res {
                    res = nums[left];
                }
                return res;
            }

            let mid = (left + right) / 2;
            if nums[mid] < res {
                res = nums[mid];
            }

            if nums[mid] >= nums[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        res
    }
}
