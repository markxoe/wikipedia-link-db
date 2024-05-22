use std::collections::{HashMap, HashSet, VecDeque};

use log::{debug, error};

use crate::remap::RemappedLinks;

pub fn find_shortest_path(start: i32, end: i32, links: &RemappedLinks) -> Option<Vec<i32>> {
    let mut queue = VecDeque::new();
    let mut predecessor = HashMap::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    visited.insert(start);

    let mut steps: u64 = 0;
    let start_time = std::time::Instant::now();
    let mut path = None;

    while let Some(at) = queue.pop_front() {
        if at == end {
            let mut out_path = VecDeque::new();
            out_path.push_front(at);
            let mut at = at;
            while let Some(&node) = predecessor.get(&at) {
                out_path.push_front(node);
                at = node;
            }
            path = Some(Vec::from(out_path));
            break;
        }

        let neighbors = links.get(&at);
        if neighbors.is_none() {
            continue;
        }
        for &neighbor in neighbors.unwrap() {
            if visited.contains(&neighbor) {
                continue;
            }

            queue.push_back(neighbor);
            visited.insert(neighbor);
            predecessor.insert(neighbor, at);
        }

        steps += 1;
        if steps % 10000 == 0 {
            if start_time.elapsed().as_secs() > 5 {
                error!("Took too long to find path, tried {steps} pages");

                break;
            }
        }
    }

    debug!("Found path in {} steps", steps);

    path
}
