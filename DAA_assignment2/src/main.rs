use std::io::{self, BufRead};

const INF: i32 = i32::MAX;

#[derive(Copy, Clone, Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    vertex: usize,
    distance: i32,
}

struct MinHeap {
    min_heap: Vec<Node>,
    positions: Vec<Option<usize>>,
}

impl MinHeap {
    pub fn new(size: usize) -> Self {
        MinHeap {
            min_heap: Vec::new(),
            positions: vec![None; size],
        }
    }

    pub fn push(&mut self, node: Node) {
        self.min_heap.push(node);
        let heap_size = self.min_heap.len() - 1;
        self.positions[node.vertex] = Some(heap_size);
    }

    pub fn pop(&mut self) -> Option<Node> {
        if self.min_heap.is_empty() {
            return None;
        }

        let root = self.min_heap[0];
        let last_element = self.min_heap.pop().unwrap();
        self.positions[root.vertex] = None;

        if !self.min_heap.is_empty() {
            self.min_heap[0] = last_element;
            self.positions[last_element.vertex] = Some(0);
            self.heapify_down(0);
        }

        Some(root)
    }

    pub fn heap_decrease_key(&mut self, vertex: usize, new_distance: i32) {
        if let Some(heap_size) = self.positions[vertex] {
            if new_distance < self.min_heap[heap_size].distance {
                self.min_heap[heap_size].distance = new_distance;
                self.heapify_up(heap_size);
            }
        }
        else {
            self.push(Node {vertex, distance: new_distance });
        }
    }

    pub fn heapify_up(&mut self, mut heap_size: usize) {
        while heap_size > 0 {
            let parent = (heap_size - 1) / 2;

            if self.min_heap[parent].distance >= self.min_heap[parent].distance {
                break;
            }

            self.min_heap.swap(parent, heap_size);
            self.positions[self.min_heap[heap_size].vertex] = Some(heap_size);
            self.positions[self.min_heap[parent].vertex] = Some(parent);

            heap_size = parent;
        }
    }

    pub fn heapify_down(&mut self, mut heap_size: usize) {
        loop {
            let mut smallest_element = heap_size;
            let left_node = 2 * heap_size + 1;
            let right_node = 2 * heap_size + 2;

            if left_node < self.min_heap.len() && self.min_heap[left_node].distance < self.min_heap[smallest_element].distance {
                smallest_element = left_node;
            }
            
            if right_node < self.min_heap.len() && self.min_heap[right_node].distance < self.min_heap[smallest_element].distance {
                smallest_element = right_node;
            }

            if smallest_element == heap_size {
                break;
            }

            self.min_heap.swap(heap_size, smallest_element);
            self.positions[self.min_heap[heap_size].vertex] = Some(heap_size);
            self.positions[self.min_heap[smallest_element].vertex] = Some(smallest_element);

            heap_size = smallest_element;
        }
    }

    // pub fn is_empty(&self) -> bool {
    //     self.min_heap.is_empty()
    // }
}

struct Graph {
    adjacency_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.adjacency_list[from].push(Edge { to, weight });
    }

    pub fn dijkstra(&self, start: usize, end: usize) -> Option<i32> {
        let mut dist = vec![INF; self.adjacency_list.len()];
        let mut min_heap = MinHeap::new(self.adjacency_list.len());

        dist[start] = 0;
        min_heap.push(Node { vertex: start, distance: 0 });

        while let Some(Node { vertex, distance }) = min_heap.pop() {
            if vertex == end {
                return Some(distance);
            }

            if distance > dist[vertex] {
                continue;
            }

            for edge in &self.adjacency_list[vertex] {
                let new_distance = distance + edge.weight;
                if new_distance < dist[edge.to] {
                    dist[edge.to] = new_distance;
                    min_heap.heap_decrease_key(edge.to, new_distance);
                }
            }
        }

        None
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} [start_vertex] [end_vertex]", args[0]);
        std::process::exit(1);
    }

    let start_vertex: usize = args[1].parse().expect("Invalid start vertex");
    let end_vertex: usize = args[2].parse().expect("Invalid end vertex");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected input"))??;
    let mut iter = first_line.split_whitespace();
    let n_vertices = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing number of vertices"))?.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let n_edges = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing number of vertices"))?.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if start_vertex > n_vertices || end_vertex > n_vertices {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Start or end vertex out of range. Total vertices: {}", n_vertices)));
    }

    let mut graph = Graph::new(n_vertices);

    for (i, line_no) in lines.enumerate() {
        let line = line_no?;
        let mut iter = line.split_whitespace();
        let vertex1: usize = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("Missing vertex1 on line {}", i + 2)))?.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let vertex2: usize = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("Missing vertex2 on line {}", i + 2)))?.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let weight: i32 = iter.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("Missing edge weight on line {}", i + 2)))?.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        graph.add_edge(vertex1, vertex2, weight);

        if i + 1 == n_edges {
            break;
        }
    }

    match graph.dijkstra(start_vertex, end_vertex) {
        Some(result) => println!("{}", result),
        None => println!("not connected"),
    }

    Ok(())
}