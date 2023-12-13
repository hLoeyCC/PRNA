// Set up the environment
mod adjacencylist_convert;
mod algorithms;
use adjacencylist_convert::RoadRecord;
use std::fs::File;
use std::io::{self, BufRead};

// The function that read the dataset and convert into vector in forms of RoadRecord for further use
fn read_road_data(filepath: &str) -> Vec<RoadRecord> {
    let file = File::open(filepath).expect("Can't open file");
    let reader = io::BufReader::new(file);

    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error on line");
        let caps: Vec<&str> = line.trim().split("\t").collect(); // data is separated by tab in dataset
        let source = caps[0].parse::<usize>().unwrap(); // the first column of data is where the connection starts
        let target = caps[1].parse::<usize>().unwrap(); // the second column is where the connection to
        records.push(RoadRecord::new(source, target)); 
    }
    records
}

fn main() {
    // import dataset "roadNet-PA.txt"
    let filepath = "roadNet-PA.txt";
    let records = read_road_data(filepath);

    // convert the dataset into adjacency list
    let nodes = adjacencylist_convert::collect_unique_nodes(&records);
    let adjacency_list = adjacencylist_convert::create_adjacency_list(&records, &nodes);

    // access basic information: mean, median, and mode 
    let (mean, median, mode, max, min) = algorithms::nodes_info(&adjacency_list);
    println!("Mean: {}, Median: {}, Mode: {}, Maximum: {}, Minimum: {}", mean, median, mode, max, min);

    // apply bfs and find the shortest path between 2 nodes (select from road NO.7 to road NO.7634)
    let shortest_path = algorithms::shortest_path_bfs(&adjacency_list,7,7634);
    println!("The Shortest Path between node NO.7 and node NO.7634 : {:?}", shortest_path);

    // find the most connected node
    let most_connected = algorithms::find_nodes_with_most_neighbors(&adjacency_list, 1); 
    println!("Most connected Node: {:?}", most_connected);

    // find the distance between 2 most connected roads
    let top_2_nodes = algorithms::find_nodes_with_most_neighbors(&adjacency_list, 2);
    let shortest_path_top_2 = algorithms::shortest_path_bfs(&adjacency_list,top_2_nodes[0],top_2_nodes[1]);
    let distance = shortest_path_top_2.expect("REASON").len();
    println!("Two roads having most number of connected roads are Road {:?} and Road {:?}",top_2_nodes[0],top_2_nodes[1]);
    println!("Distance between 2 most connected roads: {:?}",distance);  

    // find the percentage of roads going both directions
    let bi_nodes = algorithms::find_undirected_edges(&adjacency_list);
    let num_pairs = bi_nodes.len() as f64;
    let total_num_connections = algorithms::count_total_connections(&adjacency_list);
    let percentage = 100.0 * (2.0 * (num_pairs as f64)) / (total_num_connections as f64);
    println!("The number of node pairs going both directions: {:?}", num_pairs);
    println!("Percentage of node pairs going both directions: {:?}%", percentage);
}


// test section
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    // Helper function to create a specific small graph
    fn create_linear_graph() -> HashMap<usize, Vec<usize>> {
        let mut adj_list = HashMap::new();
        // Creating a smaller graph
        adj_list.insert(1, vec![2, 3]);
        adj_list.insert(2, vec![3]);
        adj_list.insert(3, vec![1, 4]);
        adj_list.insert(4, vec![]);
        adj_list
    }

    #[test]
    fn test_shortest_path_on_linear_graph() {
        let graph = create_linear_graph();
        let path = algorithms::shortest_path_bfs(&graph, 1, 4).unwrap();
        assert_eq!(path, vec![1, 3, 4]);
    }

    #[test]
    fn test_most_neighbors_nodes(){
        let graph = create_linear_graph();
        let top_node = algorithms::find_nodes_with_most_neighbors(&graph, 2);
        assert_eq!(top_node, [1, 3])
    }

    #[test]
    fn test_info_correctnness() {
        let graph = create_linear_graph();
        let (mean, median, mode, max, min) = algorithms::nodes_info(&graph);
        assert_eq!(mean, 1.25);
        assert_eq!(median, 1.5);
        assert_eq!(mode, 2.0);
        assert_eq!(max, 2.0);
        assert_eq!(min, 0.0)
    }

    #[test]
    fn test_bidirectional_path(){
        let graph = create_linear_graph();
        let bi_nodes = algorithms::find_undirected_edges(&graph);
        let num_pairs = bi_nodes.len() as f64;
        let total_num_connections = algorithms::count_total_connections(&graph);
        let percentage = 2.0 * num_pairs as f64 / (total_num_connections as f64);
        assert_eq!(percentage,0.4);
    }

}



