// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

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
