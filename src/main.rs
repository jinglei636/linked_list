use std::mem;

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val: val, next: None }
    }

    fn link(&mut self, node: Node) {
        self.next = Some(Box::new(node));
    }
}

struct LinkedList {
    head: Box<Node>,
    size: u32,
}

impl LinkedList {
    fn new() -> Self {
        let node = Node::new(0);
        LinkedList { head: Box::new(node), size: 0 }
    }

    fn push_back(&mut self, val: i32) {
        let node = Node::new(val);
        let mut curr = &mut self.head;
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }
        curr.link(node);
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<i32> {
        let mut curr = &mut self.head;
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }
        self.size -= 1;
        match mem::replace(&mut curr, None) {
            None => None,
            Some(node) => Some(node.val)
        }
    }

    fn insert() {

    }

    fn remove() {

    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(1);
    list.push_back(1);
    println!("{}",list.pop_back().unwrap());
    println!("list size: {}", list.size);
}
