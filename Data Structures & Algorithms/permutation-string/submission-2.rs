impl Solution {
    fn is_eq(vec1: Vec<char>, vec2: Vec<char>) -> bool {
        let mut hm1 = HashMap::new();
        let mut hm2 = HashMap::new();

        for e in vec1.iter() {
            let hm1_entry = hm1.entry(*e).or_insert(0);
            *hm1_entry += 1;
        }

        for e in vec2.iter() {
            let hm2_entry = hm2.entry(*e).or_insert(0);
            *hm2_entry += 1;
        }

        return hm1 == hm2;
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());

        if s1.len() > s2.len() {
            return false;
        }

        let mut left = 0;
        let mut right = left + s1.len() - 1;
        while right < s2.len() {
            if Solution::is_eq(s1.clone(), s2[left..=right].to_vec()) {
                return true;
            }

            left += 1;
            right += 1;
        }

        false
    }
}
