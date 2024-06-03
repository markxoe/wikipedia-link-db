use std::collections::VecDeque;

use regex::Regex;

use crate::{
    data::{parsers::common, redirects::Redirect},
    indication::{self, ProgressReporter},
};

fn parse_redirect_entry(line: String, (re, progress): (Regex, &ProgressReporter)) -> Vec<Redirect> {
    let mut out = vec![];

    for cap in re.captures_iter(&line) {
        out.push(Redirect {
            id: cap[1].parse::<i32>().expect("Invalid id"),
            title: cap[2].to_string(),
        })
    }

    progress.inc(1);

    out
}

pub fn read_and_parse_redirects(
    path: String,
    threads: i32,
    progress: indication::ProgressBuilder,
) -> VecDeque<Redirect> {
    let re = Regex::new(r"\(([0-9]+),0,'([^']+)','[^']*','[^']*'\)").expect("Invalid regex");

    let progress = progress
        .with_len(common::get_file_line_count(&path))
        .build();

    let out = common::parse_file_async(path, threads, parse_redirect_entry, (re, &progress));

    progress.finish();

    out
}
