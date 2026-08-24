impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut max_freq = 0;
        let mut freq = [0; 26];
        let mut left = 0;
        let mut right = 0;
        let input_vec: Vec<char> = s.chars().collect();

        while right < input_vec.len() {
            let ch = input_vec[right];
            let freq_index = ch as usize - 65;
            freq[freq_index] += 1;
            max_freq = max_freq.max(freq[freq_index]);
            let num_of_replacement = (right as i32 - left + 1) - max_freq;
            if num_of_replacement <= k {
                max_len = max_len.max(right as i32 - left + 1);
            } else {
                freq[input_vec[left as usize] as usize - 65] -= 1;
                left += 1;
            }

            right += 1;
        }

        max_len
    }
}
