impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut single_vec = Vec::new();

        for row in matrix {
            for entry in row {
                single_vec.push(entry);
            }
        }

        println!("{:?}", single_vec);

        let mut low = 0;
        let mut high = single_vec.len() - 1;

        while low <= high && low < single_vec.len() && high < single_vec.len() {
            let mid = (low + high) / 2;

            if single_vec[mid] == target {
                return true;
            } else if single_vec[mid] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        false
    }
}
