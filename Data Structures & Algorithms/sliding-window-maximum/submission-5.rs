#[derive(Debug)]
struct Data {
    value: i32,
    index: usize,
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut queue: Vec<Data> = Vec::new();
        let mut front = 0;

        for i in 0..k {
            let num = nums[i as usize];

            while queue.last().is_some() && queue.last().unwrap().value < num {
                queue.pop();
            }

            queue.push(Data {
                value: num,
                index: i as usize,
            });
        }

        res.push(queue[front].value);

        for i in k as usize..nums.len() {
            // println!("Before Queue => {:?} Front => {} I => {} Num = {}", queue, front, i, nums[i]);

            let num = nums[i];

            if queue[front].index <= i - k as usize {
                front += 1;
            }


            while queue.len() - front > 0 && queue.last().unwrap().value < num {
                queue.pop();
            }
            queue.push(Data {
                value: num,
                index: i,
            });

            // println!("After Queue => {:?} Front => {}", queue, front);
            res.push(queue[front].value);
        }
        res
    }
}
