use std::io::{self, BufRead};

const INF: i32 = i32::MAX;

struct Edge {
    to: usize,
    weight: i32,
}

struct Node {
    vertex: usize,
    distance: i32,
}

struct Heap {
    heap: Vec<Node>,
    position: Vec<i32>,
}

impl Heap {
    fn new(size: usize) -> Self {
        Heap {
            heap: Vec::new(),
            position: vec![-1; size],
        }
    }

    fn heapify_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].distance >= self.heap[parent].distance {
                break;
            }
            self.heap.swap(i, parent);
            self.position[self.heap[i].vertex] = i as i32;
            self.position[self.heap[parent].vertex] = parent as i32;
            i = parent;
        }
    }

    fn heapify_down(&mut self, mut i: usize) {
        let heap_size = self.heap.len();
        loop {
            let mut smallest_ele = i;
            let left_vertex = 2 * i + 1;
            let right_vertex = 2 * i + 2;
            if left_vertex < heap_size && self.heap[left_vertex].distance < self.heap[smallest_ele].distance {
                smallest_ele = left_vertex;
            }
            if right_vertex < heap_size && self.heap[right_vertex].distance < self.heap[smallest_ele].distance {
                smallest_ele = right_vertex;
            }
            if smallest_ele == i {
                break;
            }
            self.heap.swap(i, smallest_ele);
            self.position[self.heap[i].vertex] = i as i32;
            self.position[self.heap[smallest_ele].vertex] = smallest_ele as i32;
            i = smallest_ele;
        }
    }

    fn push(&mut self, node: Node) {
        self.heap.push(node);
        let last = self.heap.len() - 1;
        self.position[node.vertex] = last as i32;
        self.heapify_up(last);
    }

    fn pop(&mut self) -> Node {
        let root = self.heap[0];
        let last = self.heap.pop().unwrap();
        if !self.heap.is_empty() {
            self.heap[0] = last;
            self.position[self.heap[0].vertex] = 0;
            self.heapify_down(0);
        }
        self.position[root.vertex] = -1;
        root
    }

    fn heap_decrease_key(&mut self, vertex: usize, new_distance: i32) {
        let i = self.position[vertex] as usize;
        if i == usize::MAX {
            self.push(Node { vertex, distance: new_distance });
        } else if new_distance < self.heap[i].distance {
            self.heap[i].distance = new_distance;
            self.heapify_up(i);
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

struct Graph {
    vertices: usize,
    adjacency_list: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(v: usize) -> Self {
        Graph {
            vertices: v,
            adjacency_list: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.adjacency_list[from].push(Edge { to, weight });
    }

    fn dijkstra_algorithm(&self, start: usize, end: usize) -> i32 {
        let mut dist = vec![INF; self.vertices];
        dist[start] = 0;

        let mut priority_queue = Heap::new(self.vertices);
        priority_queue.push(Node { vertex: start, distance: 0 });

        while !priority_queue.is_empty() {
            let current_vertex = priority_queue.pop();
            let vertex_label = current_vertex.vertex;
            let vertex_distance = current_vertex.distance;

            if vertex_label == end {
                return vertex_distance;
            }

            if vertex_distance > dist[vertex_label] {
                continue;
            }

            for edge in &self.adjacency_list[vertex_label] {
                let edge_to = edge.to;
                let edge_weight = edge.weight;

                if dist[vertex_label] + edge_weight < dist[edge_to] {
                    dist[edge_to] = dist[vertex_label] + edge_weight;
                    priority_queue.heap_decrease_key(edge_to, dist[edge_to]);
                }
            }
        }

        -1
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [start vertex] [end_vertex] < [graph_file_name.txt]", args[0]);
        std::process::exit(1);
    }

    let start_vertex: usize = args[1].parse().expect("Invalid start vertex");
    let end_vertex: usize = args[2].parse().expect("Invalid end vertex");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n_vertex: usize = iter.next().unwrap().parse().expect("Invalid number of vertices");
    let n_edges: usize = iter.next().unwrap().parse().expect("Invalid number of edges");

    if start_vertex >= n_vertex || end_vertex >= n_vertex {
        eprintln!("Start index or end index out of range. Total vertex: {}", n_vertex);
        std::process::exit(1);
    }

    let mut graph = Graph::new(n_vertex);

    for _ in 0..n_edges {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let vertex1: usize = iter.next().unwrap().parse().expect("Invalid vertex1");
        let vertex2: usize = iter.next().unwrap().parse().expect("Invalid vertex2");
        let weight: i32 = iter.next().unwrap().parse().expect("Invalid weight");
        graph.add_edge(vertex1, vertex2, weight);
    }

    let result = graph.dijkstra_algorithm(start_vertex, end_vertex);

    if result == -1 {
        println!("not connected");
    } else {
        println!("{}", result);
    }

    Ok(())
}