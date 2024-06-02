use serde::{Deserialize, Serialize};

use crate::{indication::ProgressBuilder, ResolvedLink};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemappedLinks {
    forward: HashMap<i32, Vec<i32>>,
}

impl RemappedLinks {
    pub fn new_with_progress(
        mut links: VecDeque<ResolvedLink>,
        progress: ProgressBuilder,
    ) -> RemappedLinks {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        let progress = progress.with_len(links.len() as u64).build();

        let mut links_counter = 0;

        let (shrink_every, progress_every) = {
            let links_count = links.len();
            (links_count / 1000, links_count / 1000)
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

        RemappedLinks { forward: map }
    }

    pub fn new(links: VecDeque<ResolvedLink>) -> RemappedLinks {
        RemappedLinks::new_with_progress(links, ProgressBuilder::empty())
    }

    pub fn get(&self, from: i32) -> Option<&Vec<i32>> {
        self.forward.get(&from)
    }
}
