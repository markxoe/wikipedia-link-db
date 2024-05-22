use serde::{Deserialize, Serialize};

use crate::{lookup::PageLookup, remap::RemappedLinks};

#[derive(Serialize)]
struct SerializerDatabase<'a> {
    pub links: &'a RemappedLinks,
    pub pages: &'a PageLookup,
}

#[derive(Deserialize)]
pub struct Database {
    pub links: RemappedLinks,
    pub pages: PageLookup,
}

pub fn serialize(outfile: &str, links: &RemappedLinks, pages: &PageLookup) {
    let db = SerializerDatabase { links, pages };

    let file = std::fs::File::create(outfile).unwrap();
    let writer = std::io::BufWriter::new(file);

    // postcard::to_io(&db, writer).expect("Failed writing db");
    ciborium::into_writer(&db, writer).expect("Error writing db");
    // serde_json::to_writer(writer, &db).expect("Error writing db");
}

pub fn deserialize(infile: &str) -> Database {
    let file = std::fs::File::open(infile).unwrap();
    let reader = std::io::BufReader::new(file);

    // postcard::from_io((reader, db)).expect("Failed reading db");
    ciborium::from_reader(reader).expect("Error reading db")
    // serde_json::from_reader(reader).expect("Error reading db")
}
