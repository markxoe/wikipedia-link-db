use serde::{Deserialize, Serialize};

use crate::{data::links::LinkResolved, indication::ProgressBuilder};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkMap {
    forward: HashMap<i32, Vec<i32>>,
}

impl LinkMap {
    pub fn new_with_progress(
        mut links: VecDeque<LinkResolved>,
        progress: ProgressBuilder,
    ) -> LinkMap {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        let progress = progress.with_len(links.len() as u64).build();

        let mut links_counter = 0;

        let (shrink_every, progress_every) = {
            let links_count = links.len();
            if links_count < 1000 {
                (1, 1)
            } else {
                (links_count / 1000, links_count / 1000)
            }
        };

        while let Some((from, to)) = links.pop_front() {
            if map.contains_key(&from) {
                map.get_mut(&from).unwrap().push(to);
            } else {
                map.insert(from, vec![to]);
            }

            links_counter += 1;
            if links_counter % shrink_every == 0 {
                links.shrink_to_fit();
            }

            if links_counter % progress_every == 0 {
                progress.inc(progress_every as u64);
            }
        }

        progress.finish();

        LinkMap { forward: map }
    }

    pub fn get(&self, from: i32) -> Option<&Vec<i32>> {
        self.forward.get(&from)
    }
}

#[test]
fn new_link_map() {
    let links = VecDeque::from(vec![(1, 2), (1, 3), (3, 2)]);

    let map = LinkMap::new_with_progress(links, ProgressBuilder::empty());

    assert_eq!(map.get(1), Some(&vec![2, 3]));
    assert_eq!(map.get(2), None);
    assert_eq!(map.get(3), Some(&vec![2]));
}
