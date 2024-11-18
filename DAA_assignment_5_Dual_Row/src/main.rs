/*
    Name: Nilesh Rajiv Auradkar
    B-number: B01044448
*/

/// Represents a circuit element with position and length
#[derive(Clone)]
struct Element {
    x: i32,
    y: i32,
    length: i32,
}

/// Represents the state of element placement in dual rows
/// Stores cumulative cost and element indexes for each row
#[derive(Clone)]
struct State {
    cost: i32,
    row1_indices: Vec<usize>,
    row2_indices: Vec<usize>,
}

impl State {
    fn new() -> Self {
        State {
            cost: 0,
            row1_indices: Vec::new(),
            row2_indices: Vec::new(),
        }
    }
}

/// Calculates the cost between two points (x1, y1) and (x2, y2)
fn calculate_cost(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> i32 {
    let delta_x = (from_x - to_x).abs();
    let delta_y = (from_y - to_y).abs();
    delta_x * delta_x + delta_y * delta_y
}

/// Main legalization algorithm for dual-row circuit placement
/// Uses dynamic programming to find optimal element distribution
/// 
/// # Arguments
/// * `row1_pos`, `row2_pos` - Starting positions (x,y) for each row
/// * `elements` - Vector of circuit elements to be placed
/// 
/// # Returns
/// * Tuple of (total_cost, row1_indices, row2_indices)

fn circuit_legalization(row1_pos: (i32, i32), row2_pos: (i32, i32), elements: &[Element]) -> (i32, Vec<usize>, Vec<usize>) {
    let n = elements.len();
    let total_length: i32 = elements.iter().map(|e| e.length).sum();
    let target_length = total_length / 2;
    
    // dp[i][len] stores optimal state for elements[0..i] with row1_length = len
    let mut dp = vec![vec![None; (target_length + 1) as usize]; n + 1];
    dp[0][0] = Some(State::new());
    
    // For each element
    for i in 0..n {
        let element = &elements[i];
        
        // For each possible row1 length
        for row1_len in 0..=target_length {
            if let Some(curr_state) = dp[i][row1_len as usize].clone() {
                // Current length of elements in row2
                let row2_len: i32 = curr_state.row2_indices.iter()
                    .map(|&idx| elements[idx].length)
                    .sum();
                
                // Try placement in row1 if space available
                if row1_len + element.length <= target_length {
                    let x1 = row1_pos.0 + row1_len;
                    let cost1 = calculate_cost(element.x, element.y, x1, row1_pos.1);
                    let mut new_state = curr_state.clone();
                    new_state.cost += cost1;
                    new_state.row1_indices.push(i);
                    
                    let next_len = (row1_len + element.length) as usize;
                    // Update dp state if new cost is better
                    match &dp[i + 1][next_len] {
                        None => dp[i + 1][next_len] = Some(new_state),
                        Some(existing) if existing.cost > new_state.cost => {
                            dp[i + 1][next_len] = Some(new_state)
                        },
                        _ => {}
                    }
                }
                
                // Try placing in row 2
                if row2_len + element.length <= target_length {
                    let x2 = row2_pos.0 + row2_len;
                    let cost2 = calculate_cost(element.x, element.y, x2, row2_pos.1);
                    let mut new_state = curr_state.clone();
                    new_state.cost += cost2;
                    new_state.row2_indices.push(i);
                    
                    // Update dp state if new cost is better
                    let curr_len = row1_len as usize;
                    match &dp[i + 1][curr_len] {
                        None => dp[i + 1][curr_len] = Some(new_state),
                        Some(existing) if existing.cost > new_state.cost => {
                            dp[i + 1][curr_len] = Some(new_state)
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    
    // Find the state with the minimum cost at target_length
    let best_state = dp[n][target_length as usize]
        .as_ref()
        .expect("No valid solution found");
        
    (best_state.cost, best_state.row1_indices.clone(), best_state.row2_indices.clone())
}

#[allow(unused_mut)] // Suppresses warnings for unused Result values from read_line
fn main() {
    // Read row positions and element data from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let grids: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let row1_pos = (grids[0], grids[1]);
    
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let grids: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let row2_pos = (grids[0], grids[1]);
    
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    
    let mut elements = Vec::new();
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let grids: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        elements.push(Element {
            x: grids[0],
            y: grids[1],
            length: grids[2],
        });
    }
    
    let (cost, mut row1, mut row2) = circuit_legalization(row1_pos, row2_pos, &elements);
    
    // Output results
    println!("{}", cost);
    
    for elem in row1 {
        print!("{} ", elem);
    }
    println!();
    
    for elem in row2 {
        print!("{} ", elem);
    }
    println!();
}