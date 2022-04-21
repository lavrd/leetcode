use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

type NodeName = String;
type Weight = i128;
type Link<T> = Rc<RefCell<Node<T>>>;
type Edge<T> = (Link<T>, Weight);

// Traverse act result.
enum ActResult {
    Ok,
    Stop,
}

// TODO: Add Graph structure and store hash map with all nodes
//          to avoid useless traverse before searches.

#[derive(Debug, PartialEq)]
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

    fn add_edge(&mut self, node: Link<T>, weight: Weight) -> &mut Self {
        self.edges.push((node, weight));
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
        seen.insert(self.name.clone());
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
            seen.insert(node.name.clone());
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

// TODO: At the moment function can work only from graph root. Update to avoid this problem.
fn depth_first_topological_sort<T>(root: Link<T>) -> VecDeque<Link<T>> {
    // false - marked as temporary / true - marked as permanent.
    let marked: &mut HashMap<NodeName, (Link<T>, bool)> = &mut HashMap::new();
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
    marked: &mut HashMap<NodeName, (Link<T>, bool)>,
    sorted: &mut VecDeque<Link<T>>,
) {
    let marked_node: Option<&(Link<T>, bool)> = marked.get(&node.borrow().name);
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
    marked.insert(node.borrow().name.clone(), (node.clone(), false));
    for edge in &node.borrow().edges {
        // Visit neighbors.
        visit(edge.0.clone(), marked, sorted)
    }
    // Mark node as permanent.
    marked.insert(node.borrow().name.clone(), (node.clone(), true));
    // Add node as sorted node.
    sorted.push_front(node);
}

fn dijkstra<T>(root: Link<T>) -> (HashMap<NodeName, Weight>, HashMap<NodeName, Option<Link<T>>>)
where
    T: Clone + Eq,
{
    let mut processed: HashSet<NodeName> = HashSet::new();
    let nodes: RefCell<HashMap<NodeName, Link<T>>> = RefCell::new(HashMap::new());
    let costs: RefCell<HashMap<NodeName, Weight>> = RefCell::new(HashMap::new());
    let parents: RefCell<HashMap<NodeName, Option<Link<T>>>> = RefCell::new(HashMap::new());

    root.borrow().traverse_breadth_first(
        &|edge| -> ActResult {
            nodes.borrow_mut().insert(edge.0.borrow().name.clone(), edge.0.clone());
            costs.borrow_mut().insert(edge.0.borrow().name.clone(), Weight::MAX);
            parents.borrow_mut().insert(edge.0.borrow().name.clone(), None);
            ActResult::Ok
        },
        &mut HashSet::new(),
    );
    costs.borrow_mut().insert(root.borrow().name.clone(), 0);

    let nodes: &mut HashMap<NodeName, Link<T>> = &mut nodes.borrow_mut();
    let costs: &mut HashMap<NodeName, Weight> = &mut costs.borrow_mut();
    let parents: &mut HashMap<NodeName, Option<Link<T>>> = &mut parents.borrow_mut();

    while let Some(closest_node_name) = find_closest_node::<NodeName>(costs, &processed) {
        let closest_node: Link<T> = nodes.get(&closest_node_name).unwrap().clone();
        let cost = *costs.get_mut(&closest_node_name).unwrap();
        let edges = closest_node.borrow().edges.clone();
        for edge in edges {
            let name = edge.0.borrow().name.clone();
            let new_cost = cost + edge.1;
            let old_cost = *costs.get(&name).unwrap();
            if old_cost > new_cost {
                costs.insert(name.clone(), new_cost);
                parents.insert(name.clone(), Some(closest_node.clone()));
            }
        }
        processed.insert(closest_node.borrow().name.clone());
    }

    (costs.clone(), parents.clone())
}

fn find_closest_node<T>(
    costs: &HashMap<NodeName, Weight>,
    processed: &HashSet<NodeName>,
) -> Option<NodeName> {
    let mut closest_weight: Weight = Weight::MAX;
    let mut closest_node_name: NodeName = "".to_string();
    for (k, v) in costs.iter() {
        if *v < closest_weight && !processed.contains(k) {
            closest_weight = *v;
            closest_node_name = k.clone();
        }
    }
    if closest_node_name.is_empty() {
        return None;
    }
    Some(closest_node_name)
}

fn bellman_ford<T>(root: Link<T>) -> (HashMap<NodeName, Weight>, HashMap<NodeName, Option<Link<T>>>)
where
    T: Clone + Eq,
{
    let nodes_len: RefCell<usize> = RefCell::new(0);
    let costs: Rc<RefCell<HashMap<NodeName, Weight>>> = Rc::new(RefCell::new(HashMap::new()));
    let parents: RefCell<HashMap<NodeName, Option<Link<T>>>> = RefCell::new(HashMap::new());

    // Count nodes length and initialize hash maps.
    root.borrow().traverse_breadth_first(
        &|edge| -> ActResult {
            *nodes_len.borrow_mut() += 1;
            costs.borrow_mut().insert(edge.0.borrow().name.clone(), Weight::MAX);
            parents.borrow_mut().insert(edge.0.borrow().name.clone(), None);
            ActResult::Ok
        },
        &mut HashSet::new(),
    );
    // From root to root weight is zero.
    costs.borrow_mut().insert(root.borrow().name.clone(), 0);

    for _ in 0..*nodes_len.borrow_mut() - 1 {
        // TODO: Is it really need?
        root.borrow().traverse_breadth_first(
            &|edge| -> ActResult {
                let parent_name = edge.0.borrow().name.clone();
                let mut costs_ = costs.borrow_mut();
                let mut parents_ = parents.borrow_mut();
                // Parent cost from storage.
                let g_parent_cost: Weight = *costs_.get(&parent_name).unwrap();

                // Check each child.
                for child in edge.0.borrow().edges.iter() {
                    let child_name = child.0.borrow().name.clone();
                    // Child cost from storage.
                    let g_child_cost: Weight = *costs_.get(&child_name).unwrap();

                    // Sum parent cost from storage with current child weight and compare with child cost in storage.
                    if g_parent_cost != Weight::MAX && g_parent_cost + child.1 < g_child_cost {
                        costs_.insert(child_name.clone(), g_parent_cost + child.1);
                        // TODO: Add same parent counter.
                        //  As sometimes we have cycle or same path through all road we need to count same parents.
                        //  Or maybe not to store just parent for every node but store full path?
                        parents_.insert(child_name.clone(), Some(edge.0.clone()));
                    }
                }
                ActResult::Ok
            },
            &mut HashSet::new(),
        );
    }

    // TODO: Check negative cycles.

    let costs: HashMap<NodeName, Weight> = costs.borrow().clone();
    let parents: HashMap<NodeName, Option<Link<T>>> = parents.borrow().clone();
    (costs, parents)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rand::Rng;

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
        let mut nodes: Vec<NodeName> = Vec::new();
        for n in sorted {
            nodes.push(n.borrow().name.clone());
            print!("{} ", n.borrow().name)
        }
        println!();
        assert_eq!(nodes, vec!["R", "B", "E", "F", "A", "G", "C", "D"])
    }

    #[test]
    fn test_dijkstra() {
        let root = gen_graph();
        println!("traverse");
        root.borrow().traverse_depth_first(&print_node, &mut HashSet::new());
        let (costs, parents) = dijkstra(root.clone());
        println!("costs with root R");
        for cost in costs.iter() {
            println!("{} -> {}", cost.0, cost.1)
        }
        assert_eq!(
            costs,
            HashMap::from([
                (String::from("B"), 9),
                (String::from("E"), 14),
                (String::from("R"), 0),
                (String::from("G"), 4),
                (String::from("D"), 11),
                (String::from("A"), 1),
                (String::from("F"), 22),
                (String::from("C"), 7)
            ])
        );
        assert_eq!(*parents.get("R").unwrap(), None);
        assert_eq!(*parents.get("A").unwrap(), Some(root.clone()));
        root.borrow().traverse_breadth_first(
            &|edge| -> ActResult {
                if edge.0.borrow().name == "E" {
                    assert_eq!(*parents.get("F").unwrap(), Some(edge.0.clone()));
                    return ActResult::Stop;
                }
                ActResult::Ok
            },
            &mut HashSet::new(),
        );

        println!("traverse to root");
        let mut parent: Option<Link<u8>> =
            Some(parents.get("G").unwrap().as_ref().unwrap().clone());
        while let Some(p) = parent {
            println!("{}", p.borrow().name);
            parent = parents.get(&p.borrow().name).unwrap().clone();
        }
    }

    #[test]
    fn test_bellman_ford() {
        let root = gen_graph_bellman();
        root.borrow().traverse_breadth_first(&print_edge, &mut HashSet::new());
        let (costs, parents) = bellman_ford(root);
        println!("\ncosts - {:?}\n", costs);
        for p in parents {
            if p.1.is_none() {
                continue;
            }
            println!("parent for {} is {}", p.0, p.1.unwrap().borrow().name);
        }
    }

    #[test]
    fn test_rand() {
        let root = gen_graph_random(100, 3);
        root.borrow().traverse_breadth_first(&print_edge, &mut HashSet::new());
        let (costs, parents) = dijkstra(root.clone());
        println!("{:?}", costs);
        for p in parents {
            if p.1.is_none() {
                continue;
            }
            println!("parent for {} is {}", p.0, p.1.unwrap().borrow().name);
        }
        let (costs, parents) = bellman_ford(root);
        println!("{:?}", costs);
        for p in parents {
            if p.1.is_none() {
                continue;
            }
            println!("parent for {} is {}", p.0, p.1.unwrap().borrow().name);
        }
    }

    fn gen_graph() -> Link<u8> {
        let r = Node::new(String::from("R"), 0);
        let a = Node::new(String::from("A"), 1);
        let b = Node::new(String::from("B"), 2);
        let c = Node::new(String::from("C"), 3);
        let d = Node::new(String::from("D"), 4);
        let e = Node::new(String::from("E"), 5);
        let f = Node::new(String::from("F"), 6);
        let g = Node::new(String::from("G"), 7);

        r.borrow_mut().add_edge(Rc::clone(&a), 1).add_edge(Rc::clone(&b), 9);
        a.borrow_mut().add_edge(Rc::clone(&c), 6).add_edge(Rc::clone(&g), 3);
        b.borrow_mut().add_edge(Rc::clone(&d), 2).add_edge(Rc::clone(&e), 5);
        c.borrow_mut().add_edge(Rc::clone(&d), 7);
        e.borrow_mut().add_edge(Rc::clone(&f), 8);
        f.borrow_mut().add_edge(Rc::clone(&a), 0).add_edge(Rc::clone(&g), 4);

        r
    }

    fn gen_graph_bellman() -> Link<u8> {
        let r = Node::new(String::from("R"), 0);
        let a = Node::new(String::from("A"), 1);
        let b = Node::new(String::from("B"), 2);
        let c = Node::new(String::from("C"), 3);
        let d = Node::new(String::from("D"), 4);
        let e = Node::new(String::from("E"), 5);
        let f = Node::new(String::from("F"), 6);
        let g = Node::new(String::from("G"), 7);

        r.borrow_mut().add_edge(Rc::clone(&a), 1).add_edge(Rc::clone(&b), 9);
        a.borrow_mut().add_edge(Rc::clone(&c), 6).add_edge(Rc::clone(&g), 3);
        b.borrow_mut().add_edge(Rc::clone(&d), 2).add_edge(Rc::clone(&e), 5);
        c.borrow_mut().add_edge(Rc::clone(&d), 7);
        e.borrow_mut().add_edge(Rc::clone(&f), 8);
        f.borrow_mut().add_edge(Rc::clone(&a), -100).add_edge(Rc::clone(&g), 4);

        r
    }

    // TODO: Add max depth.
    fn gen_graph_random(max_nodes: u8, max_children: u8) -> Link<u8> {
        let mut nodes: Vec<Link<u8>> = Vec::with_capacity(max_nodes as usize);

        let r = Node::new(String::from("R"), 0);
        nodes.push(r.clone());

        for i in 0..max_nodes - 1 {
            let node_name = i.to_string().clone();
            let node = Node::new(node_name, 0);
            nodes.push(node.clone());
        }

        let mut rng = rand::thread_rng();

        for i in 0..max_nodes as usize - 1 {
            let max_children_ = rng.gen_range(1..max_children + 1);
            for _ in 0..max_children_ {
                let rand_node = rng.gen_range(0..max_nodes) as usize;
                let rand_weight: Weight = rng.gen_range(-10..10);
                nodes
                    .get(i)
                    .unwrap()
                    .borrow_mut()
                    .add_edge(nodes.get(rand_node).unwrap().clone(), rand_weight);
            }
        }

        r
    }
}
