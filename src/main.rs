// TODO: GC by stw.

use std::collections::HashMap;

struct Node<V: Copy> {
    value: V,
    key: usize,
    father: usize,
    sons: HashMap<usize, usize>,
}
type TreeNode<V> = Option<Box<Node<V>>>;

trait NodeT<V> {
    fn new(value: V, father: usize) -> Self;
}

impl<V: Copy> NodeT<V> for Node<V> {
    fn new(value: V, father: usize) -> Self {
        Node {
            value,
            key: 0,
            father,
            sons: HashMap::new(),
        }
    }
}

struct Tree<V: Copy> {
    leaves: Vec<TreeNode<V>>,
    alive: usize,
    dirt: Vec<usize>,
    gcon: bool,
}
type LinearTree<V> = Tree<V>;

trait TreeT<V> {
    fn new(value: V) -> Self;
    fn insert(&mut self, value: V, father: &mut Vec<usize>) -> usize;
    fn delete_node(&mut self, node_id: &mut Vec<usize>);
    fn remove(&mut self, node_id: &mut Vec<usize>);
    fn get(&self, node_id: &mut Vec<usize>) -> Option<V>;
    fn set(&mut self, value: V, node_id: &mut Vec<usize>);
    fn gc_on(&mut self);
    fn pre_order_node(&self, node_id: usize);
    fn pre_order(&self);
}

impl<V: Copy> TreeT<V> for Tree<V> {
    fn new(value: V) -> Self {
        let mut new = Tree {
            leaves: Vec::new(),
            alive: 0,
            dirt: Vec::new(),
            gcon: false,
        };
        new.leaves.push(Some(Box::new(Node::<V>::new(value, 0))));
        new
    }
    fn insert(&mut self, value: V, father: &mut Vec<usize>) -> usize {
        if father.len() == 0 {
            return 0;
        }
        let mut cur = father.remove(0);
        while father.len() > 0 {
            if let Some(ref n) = self.leaves[cur] {
                if let Some(new_cur) = n.sons.get(&father.remove(0)) {
                    cur = new_cur.clone();
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        println!("Inserting under: {}", cur);
        if self.dirt.len() > 0 {
            let addr = self.dirt.remove(0);
            println!("Reuse addr: {}", addr);
            if let Some(ref mut parent) = self.leaves[cur] {
                parent.sons.insert(parent.key, addr);
                parent.key += 1;
            } else {
                return 0;
            }
            if let Some(ref mut n) = self.leaves[addr] {
                n.value = value;
                n.father = cur;
                self.alive += 1;
                return addr;
            } else {
                return 0;
            }
        } else {
            let addr = self.leaves.len();
            if let Some(ref mut parent) = self.leaves[cur] {
                parent.sons.insert(parent.key, addr);
                parent.key += 1;
            } else {
                return 0;
            }
            self.leaves.push(Some(Box::new(Node::<V>::new(value, cur))));
            self.alive += 1;
            return addr;
        }
    }
    fn delete_node(&mut self, node_id: &mut Vec<usize>) {
        if node_id.len() == 0 {
            return;
        }
        let mut cur = node_id.remove(0);
        while node_id.len() > 0 {
            if let Some(ref n) = self.leaves[cur] {
                if let Some(new_cur) = n.sons.get(&node_id.remove(0)) {
                    cur = new_cur.clone();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        let mut sons = HashMap::<usize, usize>::new();
        if let Some(ref mut n) = self.leaves[cur] {
            if n.sons.len() != 0 {
                sons = n.sons.clone();
            }
            n.key = 0;
            n.sons.clear();
            self.alive -= 1;
        }
        if sons.len() != 0 {
            for (_, v) in sons {
                self.delete_node(&mut vec![v]);
            }
        }
        self.dirt.push(cur);
        println!("Delete: {}", cur);
        return;
    }
    fn remove(&mut self, node_id: &mut Vec<usize>) {
        if node_id.len() == 0 {
            return;
        }
        let mut cur = node_id.remove(0);
        while node_id.len() > 0 {
            if let Some(ref n) = self.leaves[cur] {
                if let Some(new_cur) = n.sons.get(&node_id.remove(0)) {
                    cur = new_cur.clone();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        println!("Removing: {}", cur);
        for i in 0..self.leaves.len() {
            if let Some(ref mut n) = self.leaves[i] {
                if n.sons.contains_key(&cur) {
                    n.sons.remove(&cur);
                    break;
                }
            } else {
                return;
            }
        }
        self.delete_node(&mut vec![cur]);
        if self.gcon {
            if self.alive < self.leaves.len()/2 {
                println!("GC!");
                //TODO: GC
            }
        }
    }
    fn get(&self, node_id: &mut Vec<usize>) -> Option<V> {
        if node_id.len() == 0 {
            return None;
        }
        let mut cur = node_id.remove(0);
        while node_id.len() > 0 {
            if let Some(ref n) = self.leaves[cur] {
                if let Some(new_cur) = n.sons.get(&node_id.remove(0)) {
                    cur = new_cur.clone();
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        println!("Reading: {}", cur);
        if let Some(ref n) = self.leaves[cur] {
            return Some(n.value);
        } else {
            return None;
        }
    }
    fn set(&mut self, value: V, node_id: &mut Vec<usize>) {
        if node_id.len() == 0 {
            return;
        }
        let mut cur = node_id.remove(0);
        while node_id.len() > 0 {
            if let Some(ref n) = self.leaves[cur] {
                if let Some(new_cur) = n.sons.get(&node_id.remove(0)) {
                    cur = new_cur.clone();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
        println!("Setting: {}", cur);
        if let Some(ref mut n) = self.leaves[cur] {
            n.value = value;
        } else {
            return;
        }
    }
    fn gc_on(&mut self) {
        self.gcon = true;
    }
    fn pre_order_node(&self, node_id: usize) {
        if let Some(ref n) = self.leaves[node_id] {
            println!("addr: {} father: {}", node_id, n.father);
            if n.sons.len() > 0 {
                for i in 0..n.sons.len() {
                    self.pre_order_node(n.sons[&i]);
                }
            }
        }
    }
    fn pre_order(&self) {
        self.pre_order_node(0);
    }
}

fn main() {
    let mut t = LinearTree::<i32>::new(0);
    let mut id = t.insert(1, &mut vec![0]);
    id = t.insert(2, &mut vec![id]);
    t.insert(3, &mut vec![id]);
    t.gc_on();
    t.remove(&mut vec![0, 0]);
    id = t.insert(4, &mut vec![0]);
    id = t.insert(5, &mut vec![id]);
    id = t.insert(6, &mut vec![id]);
    id = t.insert(7, &mut vec![id]);
    id = t.insert(8, &mut vec![id]);
    println!("{}", id);
    if let Some(value) = t.get(&mut vec![id]) {
        println!("{}", value);
    }
    t.set(9, &mut vec![id]);
    if let Some(value) = t.get(&mut vec![id]) {
        println!("{}", value);
    }
    t.pre_order();
}
