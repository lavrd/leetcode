use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn traverse_depth_first<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Node<T>),
    {
        if seen.contains(&self.name) {
            return;
        }
        act(self);
        seen.insert(self.name);
        for edge in &self.edges {
            edge.0.borrow().traverse_depth_first(act, seen);
        }
    }

    fn traverse_breadth_first<F>(&self, act: &F, seen: &mut HashSet<NodeName>)
    where
        F: Fn(&Edge<T>) -> ActResult,
    {
        let mut stack: VecDeque<Edge<T>> = VecDeque::new();
        stack.push_back((Rc::new(RefCell::new(self.into())), 0));
        while let Some(edge) = stack.pop_front() {
            let node = edge.0.borrow();
            if seen.contains(&node.name) {
                continue;
            }
            let res = act(&edge);
            if let ActResult::Stop = res {
                return;
            };
            seen.insert(node.name);
            for edge in &node.edges {
                stack.push_back(edge.clone())
            }
        }
    }
}

impl<T> From<&Node<T>> for Node<T>
where
    T: Clone,
{
    fn from(n: &Node<T>) -> Self {
        let name = n.name;
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
    root.borrow().traverse_breadth_first(
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

fn print_node<T>(node: &Node<T>) {
    println!("{}", node.name);
    if node.edges.is_empty() {
        println!(" -> ()");
        return;
    }
    for edge in &node.edges {
        println!(" -> {} ({})", edge.0.borrow().name, edge.1)
    }
}

fn print_edge<T>(edge: &Edge<T>) -> ActResult {
    let node = edge.0.borrow();
    println!("{}", node.name);
    if node.edges.is_empty() {
        println!(" -> ()");
        return ActResult::Ok;
    }
    for edge in &node.edges {
        println!(" -> {} ({})", edge.0.borrow().name, edge.1)
    }
    ActResult::Ok
}

fn depth_first_topological_sort<T>(root: Link<T>) -> VecDeque<Link<T>>
where
    T: Clone,
{
    // false - marked as temporary / true - marked as permanent.
    let marked: &mut HashMap<&'static str, (Link<T>, bool)> = &mut HashMap::new();
    // Sorted nodes.
    let sorted: &mut VecDeque<Link<T>> = &mut VecDeque::new();

    visit(root, marked, sorted);
    loop {
        // Get temporary marked node.
        let node: Option<Link<T>> = marked.iter().find_map(|(_, node)| {
            if !node.1 {
                return Some(node.0.clone());
            }
            None
        });
        // If there aren't any temporary nodes -> stop it.
        if node.is_none() {
            break;
        }
        // Visit next temporary node.
        visit(node.unwrap(), marked, sorted)
    }
    sorted.clone()
}

fn visit<T>(
    node: Link<T>,
    marked: &mut HashMap<&'static str, (Link<T>, bool)>,
    sorted: &mut VecDeque<Link<T>>,
) {
    let marked_node: Option<&(Link<T>, bool)> = marked.get(node.borrow().name);
    // If node is already marked.
    if let Some(node) = marked_node {
        // If node marked as permanent.
        if node.1 {
            // Stop it.
            return;
        }
        // If node already marked as temporary -> not a DAG; cycle.
        panic!("not a DAG")
    }
    // Mark node as temporary.
    marked.insert(node.borrow().name, (node.clone(), false));
    for edge in &node.borrow().edges {
        // Visit neighbors.
        visit(edge.0.clone(), marked, sorted)
    }
    // Mark node as permanent.
    marked.insert(node.borrow().name, (node.clone(), true));
    // Add node as sorted node.
    sorted.push_front(node);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_depth_first_traverse() {
        let root = gen_graph();
        print!("\n\n Depth First Traverse\n");
        root.borrow().traverse_depth_first(&print_node, &mut HashSet::new());
    }

    #[test]
    fn test_breadth_first_traverse() {
        let root = gen_graph();
        print!("\n\n Breadth First Traverse\n");
        root.borrow().traverse_breadth_first(&print_edge, &mut HashSet::new());
    }

    #[test]
    fn test_breadth_first_search() {
        let root = gen_graph();
        assert_eq!(breadth_first_search::<u8>(Rc::clone(&root), "Press F"), None);
        assert_eq!(breadth_first_search::<u8>(Rc::clone(&root), "F"), Some(6));
    }

    #[test]
    fn test_breadth_first_topology_sort() {
        let root = gen_graph();
        println!("traverse");
        root.borrow().traverse_depth_first(&print_node, &mut HashSet::new());
        let sorted = depth_first_topological_sort(root);
        println!("sorted");
        let mut nodes: Vec<&'static str> = Vec::new();
        for ele in sorted {
            nodes.push(ele.borrow().name);
            print!("{} ", ele.borrow().name)
        }
        println!();
        assert_eq!(nodes, vec!["R", "B", "E", "F", "A", "G", "C", "D"])
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
