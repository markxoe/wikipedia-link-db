use std::collections::{HashMap, HashSet, VecDeque};

use log::debug;

use crate::data::maps::link_map::LinkMap;

fn rebuild_path(predecessors: HashMap<i32, i32>, start: i32, end: i32) -> Vec<i32> {
    let mut out_path = VecDeque::new();
    out_path.push_front(end);
    let mut at = end;
    while let Some(&node) = predecessors.get(&at) {
        if at == start {
            break;
        }
        out_path.push_front(node);
        at = node;
    }

    Vec::from(out_path)
}

pub fn find_shortest_path(start: i32, end: i32, links: &LinkMap) -> Option<Vec<i32>> {
    if start == end {
        return Some(vec![start]);
    }

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
                debug!("Found path in {} steps", steps);
                return Some(rebuild_path(predecessor, start, end));
            }
        }

        steps += 1;
    }

    None
}

mod test {
    #[allow(unused_imports)]
    use crate::{data::maps::link_map::LinkMap, indication::ProgressBuilder};

    #[test]
    fn direct_link() {
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (1, 3), (3, 2)].into_iter().collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(1, 2, &link_map);

        assert_eq!(path, Some(vec![1, 2]));
    }

    #[test]
    fn start_is_end() {
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (1, 3), (3, 2)].into_iter().collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(1, 1, &link_map);

        assert_eq!(path, Some(vec![1]));
    }

    #[test]
    fn no_way() {
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (1, 3), (3, 2)].into_iter().collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(2, 1, &link_map);

        assert_eq!(path, None);
    }

    #[test]
    fn single_possibility() {
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (3, 5)]
                .into_iter()
                .collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(1, 4, &link_map);

        assert_eq!(path, Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn multiple_possibilities_one_shortest() {
        // path over 1->2->3->4 and 1->5->4
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (2, 3), (3, 4), (1, 5), (5, 4)]
                .into_iter()
                .collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(1, 4, &link_map);

        assert_eq!(path, Some(vec![1, 5, 4]));
    }

    #[test]
    fn equal_length_uses_first_in_map() {
        // path over 1->2->3->4 and 1->5->6->4
        let link_map = LinkMap::new_with_progress(
            vec![(1, 2), (2, 3), (3, 4), (1, 5), (5, 6), (6, 4)]
                .into_iter()
                .collect(),
            ProgressBuilder::empty(),
        );

        let path = super::find_shortest_path(1, 4, &link_map);

        assert_eq!(path, Some(vec![1, 2, 3, 4]));
    }
}
