impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
        let mut s1_hm = HashMap::new();

        if s1.len() > s2.len() {
            return false;
        }

        let mut left = 0;
        let mut right = left + s1.len() - 1;

        for e in s1 {
            let count = s1_hm.entry(e).or_insert(0);
            *count += 1;
        }

        while right < s2.len() {
            let mut hm = HashMap::new();
            for i in left..=right {
                let count = hm.entry(s2[i]).or_insert(0);
                *count += 1;
            }

            if s1_hm == hm {
                return true;
            }

            left += 1;
            right += 1;
        }

        false
    }
}
