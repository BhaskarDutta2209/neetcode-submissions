impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_hash_map = HashMap::new();

        for c in s.chars() {
            let count = s_hash_map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            let count = s_hash_map.entry(c).or_insert(0);
            *count -= 1;
        }

        s_hash_map.values().all(|&value| value == 0)
    }
}
