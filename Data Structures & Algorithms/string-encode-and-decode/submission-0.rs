impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();

        for str in strs {
            res.push_str(&format!("{:0>3}{}", str.len(), str)[..]);
        }

        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut st = &s[..];

        while !st.is_empty() {
            let len: usize = st[..3].parse().unwrap();
            res.push((st[3..len + 3]).to_string());
            st = &st[len + 3..];
        }

        res
    }
}
