impl Solution {
       pub fn top_k_frequent(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut frequency = HashMap::new();
        let mut bucket = vec![Vec::<i32>::new(); nums.len() + 1];
        let mut res = vec![];

        for num in nums {
            let count = frequency.entry(num).or_insert(0);
            *count += 1;
        }

        for key in frequency.keys() {
            let index = frequency.get(key).unwrap();
            bucket[*index].push(*key);
        }

        while k > 0 {
            let mut bucket_entry = bucket.pop().unwrap();
            while !bucket_entry.is_empty() && k > 0 {
                res.push(bucket_entry.pop().unwrap());
                k -= 1;
            }
        }

        res
    }

}
