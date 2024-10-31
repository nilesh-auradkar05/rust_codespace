use std::io::{self, BufRead};

const INF: i32 = i32::MAX;

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Clone)]
struct Node {
    vertex: usize,
    distance: i32,
}

struct MinHeap {
    min_heap: Vec<Node>,
    positions: Vec<i32>,
}

impl MinHeap {
    pub fn new(size: usize) -> Self {
        MinHeap {
            min_heap: Vec::new(),
            positions: vec![-1; size],
        }
    }

    pub fn push(&mut self, node: Node) {
        let vertex = node.vertex;
        self.min_heap.push(node);
        let last_element = self.min_heap.len() - 1;
        self.positions[vertex] = last_element as i32;
        self.heapify_up(last_element);
    }

    pub fn pop(&mut self) -> Option<Node> {
        if self.min_heap.is_empty() {
            return None;
        }

        let root = self.min_heap.swap_remove(0);
        self.positions[root.vertex] = -1;
        
        self.positions[self.min_heap[0].vertex] = 0;
        self.heapify_down(0);
        
        Some(root)
    }

    pub fn heapify_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;

            if self.min_heap[i].distance >= self.min_heap[parent].distance {
                break;
            }

            self.min_heap.swap(i, parent);
            self.positions[self.min_heap[i].vertex] = i as i32;
            self.positions[self.min_heap[parent].vertex] = parent as i32;

            i = parent;
        }
    }

    fn heapify_down(&mut self, mut i: usize) {
        let heap_size = self.min_heap.len();
        loop {
            let mut smallest_element = i;
            let left_vertex = 2 * i + 1;
            let right_vertex = 2 * i + 2;

            if left_vertex < heap_size && self.min_heap[left_vertex].distance < self.min_heap[smallest_element].distance
            {
                smallest_element = left_vertex;
            }
            if right_vertex < heap_size && self.min_heap[right_vertex].distance < self.min_heap[smallest_element].distance
            {
                smallest_element = right_vertex;
            }
            if smallest_element == i
            {
                break;
            }
            self.min_heap.swap(i, smallest_element);
            self.positions[self.min_heap[i].vertex] = i as i32;
            self.positions[self.min_heap[smallest_element].vertex] = smallest_element as i32;

            i = smallest_element;

        }
    }

    fn is_empty(&self) -> bool {
        self.min_heap.is_empty()
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

    fn prims_algorithm(&self, start: usize) {
        let mut visited = vec![false; self.vertices];
        let mut parent = vec![None; self.vertices];
        let mut distance_vector = vec![INF; self.vertices];

        let mut priority_queue = MinHeap::new(self.vertices);
        priority_queue.push(Node { vertex: start, distance: 0 });
        distance_vector[start] = 0;

        while !priority_queue.is_empty() {
            if let Some(current_vertex) = priority_queue.pop() {
                let vertex_label = current_vertex.vertex;

                visited[vertex_label] = true;

                for edge in &self.adjacency_list[vertex_label] {
                    let edge_to = edge.to;
                    let edge_weight = edge.weight;

                    if !visited[edge_to] && edge_weight < distance_vector[edge_to] {
                        parent[edge_to] = Some(vertex_label);
                        distance_vector[edge_to] = edge_weight;
                        priority_queue.push(Node { vertex: edge_to, distance: edge_weight });
                    }
                }
            }
        }

        let total_tree_length: i32 = distance_vector.iter().sum();
        println!("{total_tree_length}");
        
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n_vertex: usize = iter.next().unwrap().parse().expect("Missing number of vertices");
    let n_edges: usize = iter.next().unwrap().parse().expect("Missing number of edges");

    let mut graph = Graph::new(n_vertex);

    for _ in 0..n_edges {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let vertex1: usize = iter.next().unwrap().parse().expect("Missing vertex1");
        let vertex2: usize = iter.next().unwrap().parse().expect("Missing vertex2");
        let weight: i32 = iter.next().unwrap().parse().expect("Missing edge weight");
        graph.add_edge(vertex1, vertex2, weight);
    }

    graph.prims_algorithm(0);

    Ok(())
}