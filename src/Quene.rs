struct Quene<T> {
    qdata: Vec<T>,
}

impl<T> Quene<T> {
    fn new() -> Self {
        Quene {
            qdata: Vec::new(),
        }
    }

    fn push(&mut self, item:T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> T {
        self.qdata.remove(0)
    }
}

fn main() {
    let mut q = Quene::<i32>::new();
    q.push(1);
    assert_eq!(q.pop(),1);
}