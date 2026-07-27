impl Solution {
        pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut left_product = vec![1];
        let mut right_product = vec![1];

        for (index, _) in nums.iter().enumerate() {
            if index == 0 {
                continue;
            }

            left_product.push(left_product[index - 1] * nums[index - 1]);
        }

        nums.reverse();
        for (index, _) in nums.iter().enumerate() {
            if index == 0 {
                continue;
            }

            right_product.push(right_product[index - 1] * nums[index - 1]);
        }
        nums.reverse();
        right_product.reverse();

        for (index, _) in nums.iter().enumerate() {
            res.push(left_product[index] * right_product[index]);
        }

        res
    }


}
