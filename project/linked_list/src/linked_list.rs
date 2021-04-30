use std::fmt;
use std::boxed;

mod linked_list {

#[derive(Debug)]
pub struct LinkedListNode {
    data: i32;
    next: Box<LinkedListNode>;
    prev: Box<LinkedListNode>;
}
}
