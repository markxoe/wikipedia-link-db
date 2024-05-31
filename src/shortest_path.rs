use std::collections::{HashMap, HashSet, VecDeque};

use log::debug;

use crate::remap::RemappedLinks;

pub fn find_shortest_path(start: i32, end: i32, links: &RemappedLinks) -> Option<Vec<i32>> {
    let mut queue = VecDeque::new();
    let mut predecessor = HashMap::new();
    let mut visited = HashSet::new(); // note: having a set of visited nodes improves performance by a few percent while increasing memory usage
    queue.push_back(start);
    predecessor.insert(start, start);
    visited.insert(start);

    let mut steps: u64 = 0;

    while let Some(at) = queue.pop_front() {
        let neighbors = links.get(at);
        if neighbors.is_none() {
            continue;
        }
        for &neighbor in neighbors.unwrap() {
            if visited.contains(&neighbor) {
                continue;
            }

            queue.push_back(neighbor);
            predecessor.insert(neighbor, at);
            visited.insert(neighbor);

            if neighbor == end {
                let mut out_path = VecDeque::new();
                out_path.push_front(neighbor);
                let mut at = neighbor;
                while let Some(&node) = predecessor.get(&at) {
                    if at == start {
                        break;
                    }
                    out_path.push_front(node);
                    at = node;
                }

                debug!("Found path in {} steps", steps);
                return Some(Vec::from(out_path));
            }
        }

        steps += 1;
    }

    None
}
