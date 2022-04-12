use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

type NodeName = &'static str;
type Distance = u8;
type Link<T> = Rc<RefCell<Node<T>>>;
type Edge<T> = (Link<T>, Distance);

// Traverse act result.
enum ActResult {
    Ok,
    Stop,
}

struct Node<T> {
    name: NodeName,
    data: T,
    edges: Vec<Edge<T>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new(name: NodeName, data: T) -> Link<T> {
        Rc::new(RefCell::new(Node {
            name,
            data,
            edges: Vec::new(),
        }))
    }

    fn add_edge(&mut self, node: Link<T>, distance: Distance) -> &mut Self {
        self.edges.push((node, distance));
        self
    }

    fn traverse_breadth_first<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Node<T>),
    {
        if seen.contains(&self.name) {
            return;
        }
        act(self);
        seen.insert(self.name);
        for edge in &self.edges {
            edge.0.borrow().traverse_breadth_first(act, seen);
        }
    }

    fn traverse_by_level<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Edge<T>) -> ActResult,
    {
        let mut queue: VecDeque<Edge<T>> = VecDeque::new();
        queue.push_back((Rc::new(RefCell::new(self.into())), 0));
        while let Some(edge) = queue.pop_front() {
            let node = edge.0.borrow();
            if seen.contains(&node.name) {
                continue;
            }
            let res = act(&edge);
            match res {
                ActResult::Stop => return,
                _ => (),
            }
            seen.insert(&node.name);
            for edge in &node.edges {
                queue.push_back(edge.clone())
            }
        }
    }
}

impl<T> From<&Node<T>> for Node<T>
where
    T: Clone,
{
    fn from(n: &Node<T>) -> Self {
        let name = n.name.clone();
        let data = n.data.clone();
        let edges = n.edges.clone();
        Self { name, data, edges }
    }
}

fn breadth_first_search<T>(root: Link<T>, target: &str) -> Option<T>
where
    T: Clone + Eq,
{
    let found: RefCell<Option<T>> = RefCell::new(None);
    root.borrow().traverse_by_level(
        &|edge| -> ActResult {
            if edge.0.borrow().name == target {
                *found.borrow_mut() = Some(edge.0.borrow().data.clone());
                return ActResult::Stop;
            }
            ActResult::Ok
        },
        &mut HashSet::new(),
    );
    found.take()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let root = gen_graph();
        root.borrow().traverse_breadth_first(
            &|node| {
                println!("{}", node.name);
                if node.edges.is_empty() {
                    println!(" -> ()");
                    return;
                }
                for edge in &node.edges {
                    print!(" -> {} ({})\n", edge.0.borrow().name, edge.1)
                }
            },
            &mut HashSet::new(),
        );

        assert_eq!(breadth_first_search::<u8>(Rc::clone(&root), "Press F"), None);
        assert_eq!(breadth_first_search::<u8>(Rc::clone(&root), "F"), Some(6));
    }

    fn gen_graph() -> Link<u8> {
        let r = Node::new("R", 0);
        let a = Node::new("A", 1);
        let b = Node::new("B", 2);
        let c = Node::new("C", 3);
        let d = Node::new("D", 4);
        let e = Node::new("E", 5);
        let f = Node::new("F", 6);
        let g = Node::new("G", 7);

        r.borrow_mut().add_edge(Rc::clone(&a), 1).add_edge(Rc::clone(&b), 9);
        a.borrow_mut().add_edge(Rc::clone(&c), 6).add_edge(Rc::clone(&g), 3);
        b.borrow_mut().add_edge(Rc::clone(&d), 2).add_edge(Rc::clone(&e), 5);
        c.borrow_mut().add_edge(Rc::clone(&d), 7);
        e.borrow_mut().add_edge(Rc::clone(&f), 8);
        f.borrow_mut().add_edge(Rc::clone(&a), 0).add_edge(Rc::clone(&g), 4);

        r
    }
}
