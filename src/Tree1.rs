type TreeNode<V> = Option<Box<Node<V>>>;
struct Node<V: Copy> {
    key: String,
    value: V,
    sub_nodes: Vec<TreeNode<V>>,
}

trait Tree<V> {
    fn insert(&mut self, parent_key: &str, key: &str, value: V) -> Result<i32,i32>;
}

impl<V: Copy> Node<V> {
    fn new(key: &str, value: V) -> Self {
        Node {
            key: String::from(key),
            value,
            sub_nodes: Vec::new(),
        }
    }
}

impl<V: Copy> Tree<V> for Node<V> {
    fn insert(&mut self, parent_key: &str, key: &str, value: V) -> Result<i32,i32> {
        if parent_key == self.key.as_str() {
            self.sub_nodes.push(Some(Box::new(Node::new(key, value))));
            Ok(0)
        } else {
            let mut res = false;
            for i in 0..self.sub_nodes.len() {
                if let Some(ref mut n) = self.sub_nodes[i] {
                    match n.insert(parent_key, key, value.clone()) {
                        Ok(v) => {
                            res = true;
                            break;
                        },
                        Err(e) => continue,
                    }
                }
            }
            match res {
                true => Ok(0),
                false => Err(-1),
            }
        }
    }
}

type T<V> = Node<V>;

fn main() {
    let mut root = T::<i32>::new("root",0);
    let res = root.insert("root", "N1", 1);
    let res = root.insert("root", "N2", 2);
    let res = root.insert("N2", "N3", 3);
}