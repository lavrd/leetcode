use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

type NodeName = &'static str;
type Distance = u8;
type Link<T> = Rc<RefCell<Node<T>>>; // TODO: Why Rc? Why RefCell?
type Edge<T> = (Link<T>, Distance);

// Traverse step result.
enum StepResult {
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

    fn traverse<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Node<T>),
    {
        if seen.contains(&self.name) {
            return;
        }
        act(self);
        seen.insert(self.name);
        for edge in &self.edges {
            edge.0.borrow().traverse(act, seen);
        }
    }

    fn traverse_by_level<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Edge<T>) -> StepResult,
    {
        let mut queue: VecDeque<Edge<T>> = VecDeque::new();
        queue.push_back((Rc::new(RefCell::new(self.into())), 0));
        while let Some(edge) = queue.pop_front() {
            let node = edge.0.borrow();
            if seen.contains(&node.name) {
                continue;
            }
            let tsr = act(&edge);
            match tsr {
                StepResult::Stop => return,
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

// TODO: How to calculate result for one path?
fn breadth_first_search<T>(root: Link<T>, target: T) -> u16
where
    T: Clone + Eq,
{
    // TODO: Why just RefCell?
    let distance: RefCell<u16> = RefCell::new(0);
    root.borrow().traverse_by_level(
        &|edge| -> StepResult {
            println!("{:?}", edge.0.borrow().name);
            *distance.borrow_mut() += edge.1 as u16;
            if edge.0.borrow().data == target {
                return StepResult::Stop;
            }
            StepResult::Ok
        },
        &mut HashSet::new(),
    );
    let d: u16 = distance.borrow().clone();
    d
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let root = gen_graph();
        root.borrow().traverse(
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

        assert_eq!(breadth_first_search::<u8>(root, 7), 4);
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

        r.borrow_mut().add_edge(a.clone(), 1).add_edge(b.clone(), 9);
        a.borrow_mut().add_edge(c.clone(), 6).add_edge(g.clone(), 3);
        b.borrow_mut().add_edge(d.clone(), 2).add_edge(e.clone(), 5);
        c.borrow_mut().add_edge(d.clone(), 7);
        e.borrow_mut().add_edge(f.clone(), 8);
        f.borrow_mut().add_edge(a.clone(), 0).add_edge(g.clone(), 4);

        r
    }
}
