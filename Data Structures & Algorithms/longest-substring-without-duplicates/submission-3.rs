impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut hash_map = HashMap::new();
        let mut count = 0;
        let mut start = 0;

        for (index, ch) in s.chars().enumerate() {
            if let Some(x) = hash_map.insert(ch, index)
                && x >= start
            {
                start = x + 1;
                count = index - x;
                continue;
            }

            count += 1;
            if count > res {
                res = count;
            }
        }

        res as i32
    }
}
