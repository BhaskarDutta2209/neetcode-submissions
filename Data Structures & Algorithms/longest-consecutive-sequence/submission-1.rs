impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums_present = HashSet::new();
        let mut longest_sequence = 0;

        nums.iter().for_each(|n| {
            nums_present.insert(*n);
        });

        for num in nums {
            if !nums_present.contains(&(num - 1)) {
                println!("{}", num);

                // Get the length of the sequence
                let mut len = 0;
                let mut e = num;
                while nums_present.contains(&e) {
                    len += 1;
                    e += 1;
                }

                if len > longest_sequence {
                    longest_sequence = len;
                }
            }
        }

        longest_sequence
    }
}
