use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Result;
use tracing::{debug, info};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Part 1.
    Part1,

    /// Part 2.
    Part2,
}

impl Args {
    pub fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Part1 => part1(input),
            Commands::Part2 => part2(input),
        }
    }
}

fn part1(input: String) -> Result<()> {
    let maze = Maze::from_str(&input).unwrap();
    info!("{maze:?}");

    let shortest = maze.dijkstra();
    println!("{shortest}");
    Ok(())
}

fn part2(_input: String) -> Result<()> {
    Ok(())
}

struct Node {
    datum: NodeData,
    edges: Vec<(Rc<RefCell<Node>>, Direction)>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let edges: Vec<_> = self.edges.iter().map(|e| (e.0.borrow().datum.clone(), e.1)).collect();
        f.debug_struct("Node")
            .field("datum", &self.datum)
            .field("edges", &format!("{edges:?}"))
            .finish()
    }
}

impl Node {
    fn new(datum: NodeData) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            datum,
            edges: Vec::new(),
        }))
    }
}

#[derive(Debug)]
struct Graph {
    root: Rc<RefCell<Node>>,
}

#[derive(Debug, Clone)]
struct NodeData {
    pos: (usize, usize),
    kind: Kind,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Path,
    Start,
    End,
}

impl From<char> for Kind {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Path,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct NodeDistance {
    distance: Option<usize>,
    pos: (usize, usize),
    dir: Direction,
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min-heap ordering
        // None == infinity
        match (other.distance, self.distance) {
            (Some(l), Some(r)) => l.cmp(&r),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for NodeDistance {}

#[derive(Debug)]
struct Maze {
    nodes: HashMap<(usize, usize), Rc<RefCell<Node>>>,
    graph: Graph,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nodes: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter(|(_, c)| *c != '#').map(move |(x, c)| {
                    (
                        (x, y),
                        Node::new(NodeData {
                            pos: (x, y),
                            kind: Kind::from(c),
                        }),
                    )
                })
            })
            .collect();
        debug!("nodes before edge resolution: {nodes:?}");

        let dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for ((x, y), node) in &nodes {
            for dir in &dirs {
                let (dir_x, dir_y) = dir.vector();
                let edge = ((*x as isize + dir_x) as usize, (*y as isize + dir_y) as usize);
                if let Some(edge_node) = nodes.get(&edge) {
                    debug!("point:({x}, {y}) edge:{edge:?}");
                    node.borrow_mut().edges.push((edge_node.clone(), *dir));
                }
            }
        }
        debug!("nodes after edge resolution: {nodes:?}");

        let root = nodes
            .iter()
            .find(|(_, node)| matches!(node.borrow().datum.kind, Kind::Start))
            .map(|(_, node)| node)
            .unwrap()
            .clone();

        Ok(Maze {
            nodes,
            graph: Graph { root },
        })
    }
}

impl Maze {
    // for fun..
    #[allow(unused)]
    fn breadth_first_search(&self) {
        let mut visited = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_front((self.graph.root.clone(), Direction::Right));

        while let Some((node, dir)) = queue.pop_back() {
            if visited.contains(&node.borrow().datum.pos) {
                continue;
            }
            visited.insert(node.borrow().datum.pos);

            debug!("node:{:?} dir:{:?}", node.borrow().datum, dir);

            for edge in &node.borrow().edges {
                queue.push_front(edge.clone());
            }
        }
    }

    // for fun..
    #[allow(unused)]
    fn depth_first_search(&self) {
        let mut visited = HashSet::new();

        let mut stack = Vec::new();
        stack.push((self.graph.root.clone(), Direction::Right));

        while let Some((node, dir)) = stack.pop() {
            if visited.contains(&node.borrow().datum.pos) {
                continue;
            }
            visited.insert(node.borrow().datum.pos);

            debug!("node:{:?} dir:{:?}", node.borrow().datum, dir);

            for edge in &node.borrow().edges {
                stack.push(edge.clone());
            }
        }
    }

    fn dijkstra(&self) -> usize {
        let mut distances: HashMap<(usize, usize), Option<usize>> = self
            .nodes
            .iter()
            .map(|(pos, node)| match node.borrow().datum.kind {
                Kind::Start => (*pos, Some(0)),
                _ => (*pos, None),
            })
            .collect();

        let mut heap: BinaryHeap<NodeDistance> = BinaryHeap::new();
        heap.push(NodeDistance {
            pos: self.graph.root.borrow().datum.pos,
            distance: Some(0),
            dir: Direction::Right,
        });

        // predecessors here only for visualization
        let mut predecessors = HashMap::new();
        let mut visited = HashSet::new();

        let mut end_pos = None;
        while let Some(current) = heap.pop() {
            let node = self.nodes.get(&current.pos).unwrap().borrow();
            visited.insert(node.datum.pos);

            if matches!(node.datum.kind, Kind::End) {
                end_pos = Some(current.pos);
                break;
            }

            for (e_node, e_dir) in node.edges.iter() {
                let pos = e_node.borrow().datum.pos;
                if visited.contains(&pos) {
                    continue;
                }

                // use 1001 to account for rotating AND advancing
                let e_distance = if current.dir == *e_dir { 1 } else { 1001 };
                let distance = match current.distance {
                    Some(d) => d + e_distance,
                    None => e_distance,
                };
                debug!("{:?} -> {pos:?} ({distance})", current.pos);
                let update = match distances.get(&pos).unwrap() {
                    Some(d) if *d > distance => true,
                    None => true,
                    _ => false,
                };
                if update {
                    predecessors.insert(pos, current.pos);
                    *distances.get_mut(&pos).unwrap() = Some(distance);
                    heap.push(NodeDistance {
                        distance: Some(distance),
                        pos,
                        dir: *e_dir,
                    });
                }
            }
        }
        debug!("distances: {distances:?}");

        let mut pos = end_pos.unwrap();
        let mut path = vec![pos];
        while let Some(left) = predecessors.get(&pos) {
            path.push(*left);
            pos = *left;
        }

        let mut path_str = format!("{:?}", path.pop().unwrap());
        for pos in path.iter().rev() {
            path_str += &format!(" -> {pos:?}");
        }
        info!("path: {path_str}");

        distances.get(&end_pos.unwrap()).unwrap().unwrap()
    }
}
