impl Solution {
    pub fn has_cycle(mut head: *mut ListNode) -> bool {
        unsafe {
            while !head.is_null() {
                println!("Head => {:?}", (*head).val);
                if (*head).val == i32::MIN {
                    return true;
                }

                (*head).val = i32::MIN;
                head = (*head).next;
            }
        }
        false
    }
}
