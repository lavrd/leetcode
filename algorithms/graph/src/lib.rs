use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type T = &'static str;

struct Node {
    name: T,
    data: T,
    edges: Vec<Rc<RefCell<Node>>>, // TODO: Why RC? Why RefCell?
}

impl Node {
    fn new(name: T, data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name,
            data,
            edges: Vec::new(),
        }))
    }

    fn add_edge(&mut self, node: Rc<RefCell<Node>>) -> &mut Self {
        self.edges.push(node);
        self
    }

    fn traverse(&self, seen: &mut HashSet<T>) {
        if seen.contains(&self.name) {
            return;
        }
        print!("{}\n", self.name);
        for edge in &self.edges {
            print!(" -> {}\n", edge.borrow().name)
        }
        seen.insert(self.name);
        for edge in &self.edges {
            edge.borrow().traverse(seen);
        }
    }
}

fn breadth_first_search() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let root = gen_graph();
        root.borrow().traverse(&mut HashSet::new());
    }

    fn gen_graph() -> Rc<RefCell<Node>> {
        let r = Node::new("R", "");
        let a = Node::new("A", "");
        let b = Node::new("B", "");
        let c = Node::new("C", "");
        let d = Node::new("D", "");
        let e = Node::new("E", "");
        let f = Node::new("F", "");
        let g = Node::new("G", "");

        r.borrow_mut().add_edge(a.clone()).add_edge(b.clone());
        a.borrow_mut().add_edge(c.clone()).add_edge(g.clone());
        b.borrow_mut().add_edge(d.clone()).add_edge(e.clone());
        c.borrow_mut().add_edge(d.clone());
        e.borrow_mut().add_edge(f.clone());
        f.borrow_mut().add_edge(a.clone()).add_edge(g.clone());

        r
    }
}
