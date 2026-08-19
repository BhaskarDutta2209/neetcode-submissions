impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (vec1, vec2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let half_point = (vec1.len() + vec2.len() + 1) / 2;
        let is_even = (vec1.len() + vec2.len()) % 2 == 0;

        let mut left = 0;
        let mut right = vec1.len();

        while left <= right && left <= vec1.len() && right <= vec1.len() {
            let mid = left + (right - left) / 2;

            let r1 = if mid > 0 { vec1[mid - 1] } else { i32::MIN };
            let r2 = if (half_point - mid) > 0 {
                vec2[half_point - mid - 1]
            } else {
                i32::MIN
            };
            let l1 = if mid == vec1.len() {
                i32::MAX
            } else {
                vec1[mid]
            };
            let l2 = if (half_point - mid) < vec2.len() {
                vec2[half_point - mid]
            } else {
                i32::MAX
            };

            if r1 <= l2 && r2 <= l1 {
                return if is_even {
                    (std::cmp::max(r1, r2) as f64 + std::cmp::min(l1, l2) as f64) / 2f64
                } else {
                    std::cmp::max(r1, r2) as f64
                };
            } else if r1 < l2 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        0 as f64
    }
}
