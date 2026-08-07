impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if temperatures[j] < temp {
                    result[j] = (i - j) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        result
    }
}

// #[derive(Debug)]
// struct StackElement {
//     entry: i32,
//     position: usize,
// }

// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut result: Vec<i32> = vec![0; temperatures.len()];
//         let mut stack = Vec::<StackElement>::new();

//         for (index, temperature) in temperatures.iter().enumerate() {
//             if !stack.is_empty() {
//                 let mut stack_element = stack.last().unwrap();

//                 while stack_element.entry < *temperature {
//                     result[stack_element.position] =
//                         (index - stack_element.position).try_into().unwrap();

//                     stack.pop().unwrap();

//                     if stack.is_empty() {
//                         break;
//                     }
//                     stack_element = stack.last().unwrap();
//                 }
//             }

//             stack.push(StackElement {
//                 entry: *temperature,
//                 position: index,
//             });
//         }

//         while !stack.is_empty() {
//             result[stack.pop().unwrap().position] = 0;
//         }

//         result
//     }
// }
