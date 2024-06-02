use std::collections::VecDeque;

use regex::Regex;

use crate::{
    common::{self, get_file_line_count},
    indication::{ProgressBuilder, ProgressReporter},
    lookup::PageLookup,
};

type LinkResolved = (i32, i32);

fn sync_parse_link_entry(
    line: String,
    (re, resolver, progress): (Regex, &PageLookup, &ProgressReporter),
) -> Vec<LinkResolved> {
    let mut out = vec![];

    for cap in re.captures_iter(&line) {
        let (from_id, to_title) = (
            cap[1].parse::<i32>().expect("Invalid id"),
            cap[2].to_string(),
        );

        let to_id = resolver.name_to_id(&to_title);

        if let Some(to_id) = to_id {
            out.push((from_id, to_id));
        }
    }

    progress.inc(1);

    out
}

pub fn read_and_parse_links2(
    file: &str,
    threads: i32,
    resolver: &PageLookup,
    progress: ProgressBuilder,
) -> VecDeque<LinkResolved> {
    // note: namespace is fixed in regex to 0 (main namespace)
    let re = Regex::new(r"\(([0-9]+),0,'([^']+)',0,[0-9]*\)").expect("Invalid regex");

    let progress = progress.with_len(get_file_line_count(&file)).build();

    let out = common::parse_file_async(
        file.to_string(),
        threads,
        sync_parse_link_entry,
        (re, resolver, &progress),
    );

    progress.finish();

    out
}
