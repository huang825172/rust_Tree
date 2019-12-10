struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            top: None,
        }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

fn main() {
    let mut s = Stack::<i32>::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(),Some(2));
    assert_eq!(s.pop(),Some(1));
}