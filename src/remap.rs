use crate::ResolvedLink;
use std::collections::HashMap;

pub type RemappedLinks = HashMap<i32, Vec<i32>>;

pub fn remap_links(links: Vec<ResolvedLink>) -> RemappedLinks {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (from, to) in links {
        if map.contains_key(&from) {
            map.get_mut(&from).unwrap().push(to);
        } else {
            map.insert(from, vec![to]);
        }
    }

    map
}
