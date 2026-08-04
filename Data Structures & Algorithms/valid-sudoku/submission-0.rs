impl Solution {
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_hash_set: [HashSet<u32>; 9] = std::array::from_fn(|_| HashSet::new());
        let mut col_hash_set: [HashSet<u32>; 9] = std::array::from_fn(|_| HashSet::new());
        let mut matrix_hash_set: [[HashSet<u32>; 3]; 3] =
            std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new()));

        for (row_no, row) in board.iter().enumerate() {
            for (col_no, entry) in row.iter().enumerate() {
                // Check if empty or has a value
                if *entry == '.' {
                    continue;
                }

                // Change the value to int
                let entry = entry.to_digit(10).unwrap();

                // Check if existing in corresponding sets and return false if yes
                if !row_hash_set[row_no].insert(entry) {
                    return false;
                }

                if !col_hash_set[col_no].insert(entry) {
                    return false;
                }

                if !matrix_hash_set[row_no / 3][col_no / 3].insert(entry) {
                    return false;
                }
            }
        }

        true
    }

}
