/*
    Name: Nilesh Rajiv Auradkar
    B-number: B01044448
*/

use std::env;
use std::io::{self, BufRead};

// A fixed-size queue implementation for BFS traversal
/// Uses Vec as underlying storage with next and previous(prev) pointers
struct Queue {
    data: Vec<usize>,
    next: usize,
    prev: usize,
}

impl Queue {
    /// Creates a new Queue with specified capacity
    /// capacity: Maximum number of elements queue can hold
    /// Returns: Queue instance initialized with zeros
    fn new(capacity: usize) -> Self {
        Queue {
            data: vec![0; capacity],
            next: 0,
            prev: 0,
        }
    }

    fn push(&mut self, value: usize) {
        if self.prev < self.data.len() {
            self.data[self.prev] = value;
            self.prev = (self.prev + 1) % self.data.len();
        } 
    }

    /// Removes and returns element from front of queue
    /// Returns: Option<usize> which is:
    ///   - Some(value) if queue has elements
    ///   - None if queue is empty
    /// Option enum to handle nullable values safely
    fn pop(&mut self) -> Option<usize> {
        if self.next == self.prev {
            None
        }
        else {
            let value = self.data[self.next];
            self.next = (self.next + 1) % self.data.len();
            Some(value)
        }
    }

    /// Checks if queue is empty
    /// Returns: true if queue is empty
    fn is_empty(&self) -> bool {
        self.next == self.prev
    }
}

// Graph: Adjacency list representation where each vertex stores Vec of neighbors
// Capacity: Matrix storing flow capacity between vertices
type Graph = Vec<Vec<usize>>;
type Capacity = Vec<Vec<i32>>;

/// Implements Breadth-First Search for Ford-Fulkerson algorithm
/// Parameters:
///   graph: Adjacency list representation of flow network
///   capacity: Residual capacity matrix
///   source: Starting vertex
///   destination: Target vertex
///   parent: Vector to store the path
/// Returns: bool indicating if path exists from source to destination
fn bfs(graph: &Graph, capacity: &Capacity, source: usize, destination: usize, parent: &mut Vec<i32>) -> bool {
    let n = graph.len();
    parent.clear();
    parent.resize(n, -1);
    
    let mut queue = Queue::new(n);
    queue.push(source);
    parent[source] = -2;

    // BFS traversal using custom Queue
    // The `while let` pattern for handling Options value returning from functions.
    while let Some(u) = queue.pop() {
        for &v in &graph[u] {
            // Check if vertex is unvisited and has available capacity
            if parent[v] == -1 && capacity[u][v] > 0 {
                parent[v] = u as i32;
                if v == destination {
                    return true; // return true if path found.
                }
                queue.push(v);
            }
        }
    }
    false
}

fn max_flow(graph: &Graph, capacity: &mut Capacity, source: usize, destination: usize) -> i32 {
    let mut flow = 0;
    let mut parent = Vec::new();

    while bfs(graph, capacity, source, destination, &mut parent) {
        let mut path_flow = i32::MAX;
        
        let mut v = destination;
        while v != source {
            let u = parent[v] as usize;
            path_flow = path_flow.min(capacity[u][v]);
            v = u;
        }

        let mut v = destination;
        while v != source {
            let u = parent[v] as usize;
            capacity[u][v] -= path_flow;
            capacity[v][u] += path_flow;
            v = u;
        }

        flow += path_flow;
    }
    flow
}

/// Reads graph data from stdin in format:
/// - First line: n m (vertices edges)
/// - u v c (edge from u to v with capacity c)
/// Arguments: source destination (as command line args)
fn main() -> io::Result<()> {
    // Get source and destination from command line arguments
    // Parse command line using Result's ? operator for error handling if invalid input/no input.
    let args: Vec<String> = env::args().collect();

    let source: usize = args[1].parse().unwrap();
    let destination: usize = args[2].parse().unwrap();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line containing n and m
    let line = lines.next().unwrap()?;
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Initialize graph and capacity
    let mut graph = vec![Vec::new(); n];
    let mut capacity = vec![vec![0; n]; n];

    // Build graph from input edges
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let (u, v, c) = (nums[0] as usize, nums[1] as usize, nums[2]);
        graph[u].push(v);
        capacity[u][v] = c;
    }

    // Calculate and print max flow
    println!("Max flow {}", max_flow(&graph, &mut capacity, source, destination));

    Ok(())
}