type TreeNode<V> = Option<Box<Node<V>>>;
struct Node<V: Copy> {
    value: V,
    sub_nodes: Vec<usize>,
}

impl<V: Copy> Node<V> {
    fn new(value: V) -> Self {
        Node {
            value,
            sub_nodes: Vec::<usize>::new(),
        }
    }
}

type T<V> = Tree<V>;
struct Tree<V: Copy> {
    tree_nodes: Vec<TreeNode<V>>,
}

impl<V: Copy> Tree<V> {
    fn new(value: V) -> Self {
        let mut tree = Tree {
            tree_nodes: Vec::<TreeNode<V>>::new(),
        };
        tree.tree_nodes.push(Some(Box::new(Node::new(value))));
        tree
    }
}

trait TreeT<V> {
    fn insert(&mut self, parent_id: &mut Vec<usize>, value: V) -> usize;
}

impl<V: Copy> TreeT<V> for Tree<V>{
    fn insert(&mut self, parent_id: &mut Vec<usize>, value: V) -> usize{
        let mut cur = parent_id.remove(0);
        while parent_id.len()>0 {
            if let Some(ref n) = self.tree_nodes[cur] {
                cur = n.sub_nodes[parent_id.remove(0)];
            } else {
                return 0;
            }
        }
        self.tree_nodes.push(Some(Box::new(Node::new(value))));
        let len = self.tree_nodes.len();
        if let Some(ref mut parent) = self.tree_nodes[cur] {
            parent.sub_nodes.push(len-1);
            return len-1;
        } else {
            return 0;
        }
    }
}

fn main() {
    let mut tre = T::<i32>::new(0);
    let id = tre.insert(&mut vec![0], 1);
    let id = tre.insert(&mut vec![id], 1);
    let id = tre.insert(&mut vec![0,0], 1);
    let id = tre.insert(&mut vec![0,0], 1);
    let id = tre.insert(&mut vec![0,0,1], 1);
    println!("{}", id);
}