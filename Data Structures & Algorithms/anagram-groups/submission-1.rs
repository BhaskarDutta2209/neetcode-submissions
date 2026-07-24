impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<BTreeMap<char, i64>, Vec<String>> = HashMap::new();

        for str in strs {
            let wordHashmap = Solution::generate_anagram_hashmap(&str);
            let vec = anagrams.entry(wordHashmap).or_insert(vec![]);
            (*vec).push(str);
        }

        anagrams.into_values().collect()
    }

    fn generate_anagram_hashmap(str: &String) -> BTreeMap<char, i64> {
        let mut character_counts = BTreeMap::new();

        for chr in str.chars() {
            let count = character_counts.entry(chr).or_insert(0);
            *count += 1;
        }

        character_counts
    }
}
