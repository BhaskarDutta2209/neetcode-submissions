impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut low = 0;
        let mut high = numbers.len() as i32 - 1;

        while low <= high {
            let sum = numbers.get(low as usize).unwrap() + numbers.get(high as usize).unwrap();

            if sum < target {
                low += 1;
            } else if sum > target {
                high -= 1;
            } else {
                return vec![low+1, high+1];
            }
        }

        return vec![]
    }
}
