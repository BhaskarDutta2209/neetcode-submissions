impl Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut low = 0;
        let mut high = numbers.len() as i32 - 1;
        let mut ts = Vec::new();

        while low < high {
            let sum = numbers.get(low as usize).unwrap() + numbers.get(high as usize).unwrap();

            if sum < target {
                low += 1;
            } else if sum > target {
                high -= 1;
            } else {
                ts.push(vec![
                    *numbers.get(low as usize).unwrap(),
                    *numbers.get(high as usize).unwrap(),
                ]);
                low += 1;
                high -= 1;
            }
        }

        ts
    }

        pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();

        for (index, num) in nums.iter().enumerate() {
            if index > 0 && *num == nums[index-1] {
                continue;
            }
            if index < nums.len() - 2 {
                let two_sums = Solution::two_sum(nums[index + 1..].to_vec(), -1 * *num);
                if two_sums.is_empty() {
                    continue;
                }

                for two_sum in two_sums {
                    let s = vec![*num, *two_sum.first().unwrap(), *two_sum.get(1).unwrap()];
                    if res.is_empty() || *res.last().unwrap() != s {
                        res.push(s);
                    }
                }
            }
        }

        res
    }

}
