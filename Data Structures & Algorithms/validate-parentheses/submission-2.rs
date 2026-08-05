impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();

        for ch in s.chars() {
            let mut valid = true;
            match ch {
                '(' => stack.push(ch),
                ')' => {
                    let last_entry = stack.pop();
                    valid = match last_entry {
                        None => return false,
                        Some(e) => e == '(',
                    };
                }
                '[' => stack.push(ch),
                ']' => {
                    valid = match stack.pop() {
                        None => return false,
                        Some(e) => e == '[',
                    };
                }
                '{' => stack.push(ch),
                '}' => {
                    valid = match stack.pop() {
                        None => return false,
                        Some(e) => e == '{',
                    };
                }
                _ => panic!(),
            }

            if !valid {
                return false;
            }
        }

        true && stack.is_empty()
    }

}
