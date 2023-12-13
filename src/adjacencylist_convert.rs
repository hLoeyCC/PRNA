use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct RoadRecord {
    pub source: usize,  
    pub target: usize,
}

impl RoadRecord {
    pub fn new(source: usize, target: usize) -> Self {
        RoadRecord { source, target }
    }
}

// Function ensuring every node without outgoing connection is collected
pub fn collect_unique_nodes(records: &[RoadRecord]) -> HashSet<usize> {
    records.iter().fold(HashSet::new(), |mut acc, r| {
        acc.insert(r.source);
        acc.insert(r.target);
        acc
    })
}

// Function inserting evert single node and rearranging them as the form of adjacency list
pub fn create_adjacency_list(records: &[RoadRecord], nodes: &HashSet<usize>) -> HashMap<usize, Vec<usize>> {
    let mut adj_list = HashMap::new();

    for &node in nodes {
        adj_list.insert(node, Vec::new());
    }

    for record in records {
        adj_list.get_mut(&record.source).unwrap().push(record.target);
    }

    adj_list
}

