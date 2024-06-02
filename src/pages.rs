use std::collections::VecDeque;

use regex::Regex;

use crate::{
    common::{self, get_file_line_count},
    indication::{ProgressBuilder, ProgressReporter},
};

#[derive(Debug)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub redirect: bool,
}

fn parse_page_entry(line: String, (re, progressbar): (Regex, &ProgressReporter)) -> Vec<Page> {
    let mut out = vec![];

    for cap in re.captures_iter(&line) {
        out.push(Page {
            id: cap[1].parse::<i32>().expect("Invalid id"),
            title: cap[2].to_string(),
            redirect: cap[3].parse::<i32>().expect("Invalid redirect") != 0,
        })
    }

    progressbar.inc(1);

    out
}

pub fn read_and_parse_pages(
    path: String,
    threads: i32,
    progress: ProgressBuilder,
) -> VecDeque<Page> {
    let re = Regex::new(r"\(([0-9]+),0,'([^']+)',([01]),[01],[0-9.]+,'[^']*','[^']*',[0-9]*,[0-9]+,'[^']*',[^\)]*\)").expect("Invalid regex");

    let progress = progress.with_len(get_file_line_count(&path)).build();

    let out = common::parse_file_async(path, threads, parse_page_entry, (re, &progress));

    progress.finish();

    out
}
