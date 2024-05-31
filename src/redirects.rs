use std::collections::VecDeque;

use regex::Regex;

use crate::common;

#[derive(Debug)]
pub struct Redirect {
    pub id: i32,       // from
    pub title: String, // to
}

fn parse_redirect_entry(line: String, re: Regex) -> Vec<Redirect> {
    let mut out = vec![];

    for cap in re.captures_iter(&line) {
        out.push(Redirect {
            id: cap[1].parse::<i32>().expect("Invalid id"),
            title: cap[2].to_string(),
        })
    }

    out
}

pub fn read_and_parse_redirects(path: String, threads: i32) -> VecDeque<Redirect> {
    let re = Regex::new(r"\(([0-9]+),0,'([^']+)','[^']*','[^']*'\)").expect("Invalid regex");

    common::parse_file_async(path, threads, parse_redirect_entry, re)
}
