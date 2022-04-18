use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

type NodeName = &'static str;
type Weight = u8;
type Link<T> = Rc<RefCell<Node<T>>>;
type Edge<T> = (Link<T>, Weight);

// Traverse act result.
enum ActResult {
    Ok,
    Stop,
}

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
            nodes.borrow_mut().insert(edge.0.borrow().name, edge.0.clone());
            costs.borrow_mut().insert(edge.0.borrow().name, Weight::MAX);
            parents.borrow_mut().insert(edge.0.borrow().name, None);
            ActResult::Ok
        },
        &mut HashSet::new(),
    );
    costs.borrow_mut().insert(root.borrow().name, 0);

    let nodes: &mut HashMap<NodeName, Link<T>> = &mut nodes.borrow_mut();
    let costs: &mut HashMap<NodeName, Weight> = &mut costs.borrow_mut();
    let parents: &mut HashMap<NodeName, Option<Link<T>>> = &mut parents.borrow_mut();

    while let Some(closest_node_name) = find_closest_node::<NodeName>(costs, &processed) {
        let closest_node: Link<T> = nodes.get(closest_node_name).unwrap().clone();
        let cost = *costs.get_mut(closest_node_name).unwrap();
        let edges = closest_node.borrow().edges.clone();
        for edge in edges {
            let name = edge.0.borrow().name;
            let new_cost = cost + edge.1;
            let old_cost = *costs.get(name).unwrap();
            if old_cost > new_cost {
                costs.insert(name, new_cost);
                parents.insert(name, Some(closest_node.clone()));
            }
        }
        processed.insert(closest_node.borrow().name);
    }

    (costs.clone(), parents.clone())
}

fn find_closest_node<T>(
    costs: &HashMap<NodeName, Weight>,
    processed: &HashSet<NodeName>,
) -> Option<NodeName> {
    let mut closest_weight: Weight = Weight::MAX;
    let mut closest_node_name: NodeName = "";
    for (k, v) in costs.iter() {
        if *v < closest_weight && !processed.contains(k) {
            closest_weight = *v;
            closest_node_name = k;
        }
    }
    if closest_node_name.is_empty() {
        return None;
    }
    Some(closest_node_name)
}

fn bellman_ford<T>(
    root: Link<T>,
) -> (HashMap<NodeName, HashMap<NodeName, Weight>>, HashMap<NodeName, Option<Link<T>>>)
    where
        T: Clone + Eq,
{
    let nodes: RefCell<HashMap<NodeName, Link<T>>> = RefCell::new(HashMap::new());
    let costs: Rc<RefCell<HashMap<NodeName, HashMap<NodeName, Weight>>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let parents: RefCell<HashMap<NodeName, Option<Link<T>>>> = RefCell::new(HashMap::new());

    root.borrow().traverse_breadth_first(
        &|edge| -> ActResult {
            nodes.borrow_mut().insert(edge.0.borrow().name, edge.0.clone());
            costs.borrow_mut().insert(edge.0.borrow().name, HashMap::new());
            parents.borrow_mut().insert(edge.0.borrow().name, None);
            ActResult::Ok
        },
        &mut HashSet::new(),
    );

    for d in costs.borrow_mut().iter_mut() {
        for n in nodes.borrow().iter() {
            d.1.insert(n.0, Weight::MAX);
        }
    }
    costs.borrow_mut().get_mut(root.borrow().name).unwrap().insert(root.borrow().name, 0);

    root.borrow().traverse_breadth_first(
        &|edge| -> ActResult {
            let mut _costs = costs.borrow_mut();
            let parent_name = edge.0.borrow().name;
            let parent_costs = _costs.get_mut(parent_name).unwrap();
            let g_parent_cost: Weight = *parent_costs.get_mut(parent_name).unwrap();

            for child in edge.0.borrow().edges.iter() {
                let child_name = child.0.borrow().name;
                let g_child_cost: Weight = *parent_costs.get_mut(child_name).unwrap();

                if g_parent_cost != Weight::MAX && g_parent_cost + edge.1 < g_child_cost {
                    // _costs.get_mut(child_name).unwrap().insert(child_name, g_parent_cost + edge.1);
                    // edge.0.borrow().edges
                }
            }

            unreachable!()
        },
        &mut HashSet::new(),
    );

    // let mut costs = costs.borrow_mut();

    // for _ in 0..nodes.len() - 1 {
    //     for (parent_name, parent_costs) in costs.clone().iter() {
    //         for (child_name, child_cost) in parent_costs.iter() {
    //             let g_parent_cost: Weight =
    //                 *costs.get_mut(parent_name).unwrap().get(parent_name).unwrap();
    //             let g_child_cost: Weight =
    //                 *costs.get_mut(parent_name).unwrap().get(child_name).unwrap();
    //             if g_parent_cost != Weight::MAX && g_parent_cost + child_cost < g_child_cost {
    //                 println!("OK");
    //                 // TODO: Pass parent here.
    //                 costs
    //                     .get_mut(child_name)
    //                     .unwrap()
    //                     .insert(child_name, g_parent_cost + child_cost);
    //             }
    //         }
    //     }
    // }
    //
    // // TODO: Check negative cycles.
    //
    // (costs.clone(), parents.clone())
    unreachable!()
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
        let mut nodes: Vec<NodeName> = Vec::new();
        for n in sorted {
            nodes.push(n.borrow().name);
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
                ("B", 9),
                ("E", 14),
                ("R", 0),
                ("G", 4),
                ("D", 11),
                ("A", 1),
                ("F", 22),
                ("C", 7)
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
        let mut parent: Option<Link<u8>> = Some(parents.get("G").unwrap().as_ref().unwrap().clone());
        while let Some(p) = parent {
            println!("{}", p.borrow().name);
            parent = parents.get(p.borrow().name).unwrap().clone();
        }
    }

    #[test]
    fn test_bellman_ford() {
        let root = gen_graph();
        let (costs, _parents) = bellman_ford(root);
        println!("{:?}", costs)
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
