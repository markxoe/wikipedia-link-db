use serde::{Deserialize, Serialize};

use crate::ResolvedLink;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemappedLinks {
    forward: HashMap<i32, Vec<i32>>,
}

impl RemappedLinks {
    pub fn new(mut links: VecDeque<ResolvedLink>) -> RemappedLinks {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut links_counter = 0;

        let shrink_every = {
            let links_count = links.len();
            links_count / 1000
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
        }

        RemappedLinks { forward: map }
    }

    pub fn get(&self, from: i32) -> Option<&Vec<i32>> {
        self.forward.get(&from)
    }
}
