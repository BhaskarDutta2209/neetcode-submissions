impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn healper(
            head: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match head {
                None => prev,
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = prev;
                    healper(next, Some(node))
                }
            }
        }

        healper(head, None)
    }
}
