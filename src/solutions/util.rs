use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl Debug for ListNode<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.val, if let Some(ref x) = self.next {format!("{x:?}")} else {"!".to_string()}))
    } 
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
    pub fn new_boxed(val: T) -> Option<Box<Self>> {
        Some(Box::new(ListNode::new(val)))
    }
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut it = vec.into_iter();
        let mut node = if let Some(val) = it.next() {
            ListNode::new(val)
        } else {
            panic!("Linked List Expects a Value");
        };
        for elem in it {
            node.append_inner(elem);
        }
        node
    }
    fn append_inner(&mut self, val: T) {
        if let Some(ref mut node) = self.next {
            node.append_inner(val)
        } else {
            self.next = ListNode::new_boxed(val)
        }
    }
}
