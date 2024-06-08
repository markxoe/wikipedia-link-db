use serde::{Deserialize, Serialize};

use crate::data::maps::{link_map::LinkMap, page_map::PageMap};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub links: LinkMap,
    pub pages: PageMap,
}

impl Database {
    pub fn new(links: LinkMap, pages: PageMap) -> Self {
        Self { links, pages }
    }

    pub fn to_file(&self, outfile: &str) {
        let file = std::fs::File::create(outfile).unwrap();
        let writer = std::io::BufWriter::new(file);

        ciborium::into_writer(self, writer).expect("Error writing db");
    }

    pub fn from_file(infile: &str) -> Database {
        let file = std::fs::File::open(infile).unwrap();
        let reader = std::io::BufReader::new(file);

        ciborium::from_reader(reader).expect("Error reading db")
    }
}
