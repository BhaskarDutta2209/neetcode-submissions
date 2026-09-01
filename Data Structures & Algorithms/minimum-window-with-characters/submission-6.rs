impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut found = false;
        let mut starting_index = 0;
        let mut left = 0;
        let mut right = 0;
        let mut min_len = usize::MAX;
        let mut freq = HashMap::new();
        let mut match_count = 0;

        for ch in t_chars {
            let count = freq.entry(ch).or_insert(0);
            *count += 1;
        }

        while right < s.len() {
            let count = freq.entry(s_chars[right]).or_insert(0);
            if *count > 0 {
                match_count += 1;
            }
            *count -= 1;
            right += 1;
            while match_count == t.len() {
                if right - left < min_len {
                    found = true;
                    starting_index = left;
                    min_len = right - left;
                }

                let count = freq.entry(s_chars[left]).or_insert(0);
                *count += 1;
                if *count > 0 {
                    match_count -= 1;
                }
                left += 1;
            }
        }

        if found {
            let end_index = starting_index + min_len;
            s[starting_index..end_index].to_string()
        } else {
            "".to_string()
        }
    }
}
