impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right && left < nums.len() && right < nums.len() {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] == target {
                return left as i32;
            }

            if nums[right] == target {
                return right as i32;
            }

            if nums[left] >= nums[right] {
                if (nums[mid] < nums[right] && target > nums[right]) {
                    right = mid - 1;
                } else if nums[mid] < nums[right] && target > nums[mid] {
                    left = mid + 1;
                } else if nums[mid] < nums[right] {
                    right = mid - 1;
                } else if target > nums[left] && target > nums[mid] {
                    left = mid + 1;
                } else if target > nums[left] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }

                // if (nums[right] > target && nums[right] > nums[mid]) || (nums[right] < target && nums[mid] > target) {
                //     right = mid  - 1;
                //     // left = mid + 1
                // } else {
                //     left = mid + 1;
                //     // right = mid - 1;
                // }
            } else {
                if nums[mid] > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        -1
    }
}
