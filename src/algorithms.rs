use std::collections::{VecDeque, HashSet, HashMap};

// Function for finding the shortest path between 2 nodes
pub fn shortest_path_bfs(adj_list: &HashMap<usize, Vec<usize>>, start: usize, end: usize) -> Option<Vec<usize>> {
    // apply the method of breadth first search
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut prev = vec![None; adj_list.keys().max().unwrap() + 1]; 

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = adj_list.get(&node) {
            for &adj in neighbors {
                if !visited.contains(&adj) {
                    visited.insert(adj);
                    prev[adj] = Some(node);
                    queue.push_back(adj);

                    if adj == end {
                        let mut path = Vec::new();
                        let mut current = Some(end);
                        while let Some(node) = current {
                            path.push(node);
                            current = prev[node];
                        }
                        path.reverse();
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

// Function for finding any number of most connected nodes
pub fn find_nodes_with_most_neighbors(adj_list: &HashMap<usize, Vec<usize>>, num_nodes: usize) -> Vec<usize> {
    let mut num_neighbors_list = adj_list.iter()
                              .map(|(&node, connections)| (node, connections.len()))
                              .collect::<Vec<(usize, usize)>>();

    // reurn empty vector if the adjacency list is empty
    if num_nodes == 0 || num_neighbors_list.is_empty() {
        return Vec::new();
    }

    // Sort the list of number of connections each node has
    num_neighbors_list.sort_by(|a, b| b.1.cmp(&a.1));

    // Select the top `num_nodes`, or all roads if there are fewer than `num_roads`
    let mut final_list = num_neighbors_list.iter()
                                        .take(num_nodes)
                                        .map(|&(node, _)| node)
                                        .collect::<Vec<usize>>();
    final_list.sort(); // return the outcome in order if there're more than one
    final_list 
}


// Function for finding the basic information of this data set: Mean, Median, and Mode
pub fn nodes_info(adj_list: &HashMap<usize, Vec<usize>>) -> (f64, f64, f64, f64, f64) {
    let mut num_connections: Vec<usize> = adj_list.values()
                                                   .map(|roads| roads.len())
                                                   .collect();

    if num_connections.is_empty() {
        return (0.0, 0.0, 0.0, 0.0, 0.0); // No roads in the graph
    }

    // Calculate mean
    let total_connections: usize = num_connections.iter().sum();
    let mean = total_connections as f64 / num_connections.len() as f64;

    // Calculate median
    num_connections.sort_unstable();
    let mid = num_connections.len() / 2;
    let median = if num_connections.len() % 2 == 0 {
        (num_connections[mid - 1] + num_connections[mid]) as f64 / 2.0
    } else {
        num_connections[mid] as f64
    };

    // calculate mode
    let mut map = HashMap::new();

    for &connections in &num_connections {
        *map.entry(connections).or_insert(0) += 1;
    }

    let mode = map.into_iter()
                 .max_by_key(|&(_, count)| count)
                 .map(|(length, _)| length as f64)
                 .unwrap_or(0.0);

    // calculate the highest & lowest
    let max = *num_connections.iter().max().unwrap() as f64;
    let min = *num_connections.iter().min().unwrap() as f64;


    (mean, median, mode, max, min)
}

// Function for finding the number of both direction node pairs (e.g. A <-> B)
pub fn find_undirected_edges(adj_list: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let mut bidirectional_roads = Vec::new();

    for (&road, connections) in adj_list {
        for &connected_road in connections {
            // Only process the pair if the current road has a smaller index than the connected road
            if road < connected_road {
                if let Some(connected_roads) = adj_list.get(&connected_road) {
                    if connected_roads.contains(&road) {
                        bidirectional_roads.push((road, connected_road));
                    }
                }
            }
        }
    }

    bidirectional_roads
}

// Function for finding the total number of connections within the data set
pub fn count_total_connections(adj_list: &HashMap<usize, Vec<usize>>) -> usize {
    adj_list.values()
            .map(|connections| connections.len())
            .sum()
}