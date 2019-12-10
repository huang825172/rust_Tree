mod tree {
    pub struct Node {
        name: String,
        data: String,
        sub_nodes: Vec<Node>,
    }

    impl Node {
        pub fn get_node_name(&self) -> &String {
            &self.name
        }
    }

    pub struct Tree {
        name: String,
        root_node: Node,
    }

    pub fn new_tree(name: String) -> Tree {
        Tree {
            name,
            root_node: Node {
                name: String::from("root"),
                data: String::from(""),
                sub_nodes: Vec::new(),
            }
        }
    }

    fn get_sub_node<'a>(node: &'a mut Node, name: &String) -> &'a mut Node {
        let node_name = String::from(name);
        for item in node.sub_nodes.iter() {
            if item.name == node_name {
                return &mut item;
            }
            let sub = get_sub_node(&mut item, name);
            if sub.name != item.name {
                return &mut sub;
            }
        }
        node
    }

    fn get_sub_node_n<'a>(node: &'a  Node, name: &String) -> &'a Node {
        let node_name = String::from(name);
        for item in node.sub_nodes.iter() {
            if item.name == node_name {
                return &item;
            }
            let sub = get_sub_node_n(&item, name);
            if sub.name != item.name {
                return &sub;
            }
        }
        node
    }

    impl Tree {
        pub fn add_node(&mut self,
            parent: String,
            name: String, data: String) {
            if parent == "root" {
                self.root_node.sub_nodes.push(Node {
                    name,
                    data,
                    sub_nodes: Vec::new(),
                });
                return;
            }
            let res = get_sub_node(&mut self.root_node, &name);
            if res.name != "root" {
                res.sub_nodes.push(Node {
                    name,
                    data,
                    sub_nodes: Vec::new(),
                });
            }
        }

        pub fn get_tree_name(&self) -> &String {
            &self.name
        }

        pub fn get_node(&self,
            name: &String) -> &Node {
                let res = get_sub_node_n(&self.root_node, name);
                if res.name != "root" {
                    return res;
                }
                &self.root_node
            }
    }
}

fn main() {
    let mut tree = tree::new_tree(String::from("T1"));
    tree.add_node(String::from("root"),
        String::from("N1"), String::from("Data"));
    println!("{}",tree.get_tree_name());
    println!("{}",tree.get_node(&String::from("N1")).get_node_name());
}
