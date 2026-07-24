impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut encountered_entries = HashMap::new();
        for num in nums {
            if encountered_entries.contains_key(&num) {
                return true;
            }
            encountered_entries.insert(num, true);
        }

        false
    }
}
