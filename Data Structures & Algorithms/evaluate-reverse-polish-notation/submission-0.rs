impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let res = v1 + v2;
                    stack.push(res);
                }
                "-" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let res = v2 - v1;
                    stack.push(res);
                }
                "*" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let res = v2 * v1;
                    stack.push(res);
                }
                "/" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let res = v2 / v1;
                    stack.push(res);
                }
                num => {
                    let n: i32 = num.parse().unwrap();
                    stack.push(n);
                }
            }
        }

        *stack.last().unwrap()
    }

}
