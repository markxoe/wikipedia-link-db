use serde::{Deserialize, Serialize};

use crate::data::maps::{link_map::LinkMap, page_map::PageMap};

#[derive(Serialize)]
struct SerializerDatabase<'a> {
    pub links: &'a LinkMap,
    pub pages: &'a PageMap,
}

#[derive(Deserialize)]
pub struct Database {
    pub links: LinkMap,
    pub pages: PageMap,
}

pub fn serialize(outfile: &str, links: &LinkMap, pages: &PageMap) {
    let db = SerializerDatabase { links, pages };

    let file = std::fs::File::create(outfile).unwrap();
    let writer = std::io::BufWriter::new(file);

    ciborium::into_writer(&db, writer).expect("Error writing db");
}

pub fn deserialize(infile: &str) -> Database {
    let file = std::fs::File::open(infile).unwrap();
    let reader = std::io::BufReader::new(file);

    ciborium::from_reader(reader).expect("Error reading db")
}
