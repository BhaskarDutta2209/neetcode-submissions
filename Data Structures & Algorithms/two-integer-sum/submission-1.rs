impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_set = HashMap::new();

        // Store the vec into a HashMap
        for (index, num) in nums.iter().enumerate() {
            num_set.insert(num, index);
        }

        // Reiterate to find the indexes
        for (index, num) in nums.iter().enumerate() {
            let diff = target - num;
            match num_set.get(&diff) {
                Some(val) => {
                    let first_index: i32 = index.try_into().unwrap();
                    let second_index: i32 = (*val).try_into().unwrap();
                    if first_index == second_index {
                        continue;
                    } else {
                        return Vec::from([first_index, second_index]);
                    }
                }
                None => continue,
            }
        }

        vec![]
    }


}
