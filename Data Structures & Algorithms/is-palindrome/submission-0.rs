impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let input_chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_ascii_alphanumeric())
            .collect();

        let len = input_chars.len();

        for (index, ch) in input_chars.iter().enumerate() {
            if *ch != input_chars[len - index - 1] {
                return false;
            }
        }

        true
    }
}
