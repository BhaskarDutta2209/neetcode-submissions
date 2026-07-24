use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};

// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         let mut s_hash_map = HashMap::new();
//         let mut t_hash_map = HashMap::new();

//         for c in s.chars() {
//             let count = s_hash_map.entry(c).or_insert(0);
//             *count += 1;
//         }

//         for c in t.chars() {
//             let count = t_hash_map.entry(c).or_insert(0);
//             *count += 1;
//         }

//         s_hash_map == t_hash_map
//     }
// }

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_hash_map = HashMap::new();

        for c in s.chars() {
            let count = s_hash_map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            match s_hash_map.entry(c) {
                Occupied(e) => {
                    if *e.get() > 1 {
                        let value = e.into_mut();
                        *value -= 1;
                    } else {
                        e.remove_entry();
                    }
                }
                Vacant(_) => {
                    return false;
                }
            }
        }

        s_hash_map.is_empty()
    }
}
