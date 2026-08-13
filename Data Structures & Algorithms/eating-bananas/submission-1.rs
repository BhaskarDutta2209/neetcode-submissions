impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_k = *piles.iter().max().unwrap();

        let mut min_k = max_k;

        let mut low = 1;
        let mut high = max_k;

        while low <= high && low <= max_k && high <= max_k {
            let mid = low + ((high - low) / 2);
            let mut total_time = 0;
            piles.iter().for_each(|pile| {
                total_time += (*pile as f64 / mid as f64).ceil() as i32;
            });
            if total_time > h {
                low = mid + 1;
            } else {
                if mid < min_k {
                    min_k = mid;
                }

                high = mid - 1;
            }
        }

        min_k
    }
}
