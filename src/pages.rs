use regex::Regex;

use crate::common;

#[derive(Debug)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub redirect: bool,
}

fn parse_page_entry(line: String, re: Regex) -> Vec<Page> {
    let mut out = vec![];

    for cap in re.captures_iter(&line) {
        let namespace = cap[2].parse::<i32>().expect("Invalid namespace");
        if namespace != 0 {
            continue;
        }
        out.push(Page {
            id: cap[1].parse::<i32>().expect("Invalid id"),
            title: cap[3].to_string(),
            redirect: cap[4].parse::<i32>().expect("Invalid redirect") != 0,
        })
    }

    out
}

pub fn read_and_parse_pages(path: String, threads: i32) -> Vec<Page> {
    let re = Regex::new(r"\(([0-9]+),([0-9]+),'([^']+)',([01]),[01],[0-9.]+,'[^']*','[^']*',[0-9]*,[0-9]+,'[^']*',[^\)]*\)").expect("Invalid regex");

    common::parse_file_async(path, threads, parse_page_entry, re)
}
