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

    fn have_sub(&self, key: usize) -> Option<usize> {
        for i in 0..self.sub_nodes.len() {
            if self.sub_nodes[i]== key {
                return Some(i);
            }
        }
        None
    }
}

type T<V> = Tree<V>;
struct Tree<V: Copy> {
    tree_nodes: Vec<TreeNode<V>>,
    trash_bin: Vec<usize>,
}

impl<V: Copy> Tree<V> {
    fn new(value: V) -> Self {
        let mut tree = Tree {
            tree_nodes: Vec::<TreeNode<V>>::new(),
            trash_bin: Vec::<usize>::new(),
        };
        tree.tree_nodes.push(Some(Box::new(Node::new(value))));
        tree
    }
}

trait TreeT<V> {
    fn insert(&mut self, parent_id: &mut Vec<usize>, value: V) -> usize;
    fn get_address(&self, node_id: usize) -> Vec<usize>;
    fn delete(&mut self, node_id: &mut Vec<usize>);
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
        if self.trash_bin.len()>0 {
            let new_cur = self.trash_bin.remove(0);
            if let Some(ref mut n) = self.tree_nodes[new_cur] {
                n.value = value;
                n.sub_nodes.clear();
            } else {
                return 0;
            }
            if let Some(ref mut parent) = self.tree_nodes[cur] {
                parent.sub_nodes.push(new_cur);
                return new_cur;
            } else {
                return 0;
            }
        } else {
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

    fn get_address(&self, node_id: usize) -> Vec<usize> {
        let mut addr_rev = Vec::<usize>::new();
        let mut target = node_id;
        let n_num = self.tree_nodes.len();
        while target!=0 {
            for i in 0..n_num {
                if let Some(ref n) = self.tree_nodes[i] {
                    if let Some(n) = n.have_sub(target) {
                        addr_rev.push(n);
                        target = i;
                        break;
                    }
                } else {
                    return vec![0];
                }
            }
        }
        let mut addr = Vec::<usize>::new();
        while addr_rev.len()>0 {
            addr.push(addr_rev.remove(0));
        }
        addr
    }

    fn delete(&mut self, node_id: &mut Vec<usize>) {
        let mut cur = node_id.remove(0);
        while node_id.len()>0 {
            if let Some(ref n) = self.tree_nodes[cur] {
                cur = n.sub_nodes[node_id.remove(0)];
            } else {
                return;
            }
        }
        let mut sub_n: Vec<usize>;
        if let Some(ref mut n) = self.tree_nodes[cur] {
            sub_n = n.sub_nodes.clone();
        } else {
            return;
        }
        while sub_n.len()>0 {
            self.delete(&mut vec![sub_n.remove(0)]);
        }
        self.trash_bin.push(cur);
    }
}

fn main() {
    let mut tre = T::<i32>::new(0);
    let id = tre.insert(&mut vec![0], 1);
    let id = tre.insert(&mut vec![0,0], 2);
    let id = tre.insert(&mut vec![0,0], 3);
    let id = tre.insert(&mut vec![0], 4);
    let id = tre.insert(&mut vec![0,1], 5);
    let id = tre.insert(&mut vec![0,1,0], 6);
    println!("{}", id);
    tre.delete(&mut vec![0,0]);
    let id = tre.insert(&mut vec![0], 1);
    let id = tre.insert(&mut vec![0,0], 2);
    let id = tre.insert(&mut vec![0,0], 3);
    println!("{}", id);
    let id = tre.insert(&mut vec![0,0,0], 7);
    let id = tre.insert(&mut vec![0,1,0,0], 8);
    println!("{}", id);
}