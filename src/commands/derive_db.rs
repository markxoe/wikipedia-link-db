use clap::Args;
use log::info;

use crate::{
    data::{
        database,
        maps::{link_map::LinkMap, page_map::PageMap},
        parsers::{links, pages, redirects},
    },
    indication::ProgressBuilder,
};

use super::ArgExecutor;

#[derive(Args, Debug, Clone)]
#[group()]
pub struct DeriveDbArgs {
    /// Path to the page.sql file
    #[arg(short, long)]
    page_sql: String,

    /// Path to the redirects.sql file
    #[arg(short, long)]
    redirect_sql: String,

    /// Path to the pagelinks.sql file
    #[arg(short = 'l', long)]
    pagelinks_sql: String,

    /// Output Path
    #[arg(short, long)]
    output: String,

    /// Number of threads to use
    #[arg(short, long, default_value = "2")]
    threads: i32,
}

impl ArgExecutor for DeriveDbArgs {
    fn execute(&self) {
        derive_db_command(self.clone());
    }
}

fn derive_db_command(args: DeriveDbArgs) {
    let (page_sql, redirect_sql, pagelinks_sql, output, threads) = (
        args.page_sql,
        args.redirect_sql,
        args.pagelinks_sql,
        args.output,
        args.threads,
    );

    let (pages, redirects) = {
        let pages = pages::read_and_parse_pages(
            page_sql,
            threads,
            ProgressBuilder::new()
                .with_steps(1, 6)
                .with_message("Loading pages...")
                .with_finish_message("Pages loaded"),
        );
        let redirects = redirects::read_and_parse_redirects(
            redirect_sql,
            threads,
            ProgressBuilder::new()
                .with_steps(2, 6)
                .with_message("Loading redirects...")
                .with_finish_message("Redirects loaded"),
        );

        (pages, redirects)
    };

    info!(
        "Got {} pages and {} redirects",
        pages.len(),
        redirects.len()
    );

    let lookup = PageMap::new_with_progress(
        pages,
        redirects,
        ProgressBuilder::new()
            .with_steps(3, 6)
            .with_message("Remapping pages...")
            .with_finish_message("Pages remapped"),
    );

    let links = links::read_and_parse_links(
        pagelinks_sql.as_str(),
        threads,
        &lookup,
        ProgressBuilder::new()
            .with_steps(4, 6)
            .with_message("Loading links...")
            .with_finish_message("Links loaded"),
    );

    let links = LinkMap::new_with_progress(
        links,
        ProgressBuilder::new()
            .with_steps(5, 6)
            .with_message("Remapping links...")
            .with_finish_message("Links remapped"),
    );

    {
        let spinner = ProgressBuilder::spinner()
            .with_message("Serializing and writing file")
            .with_steps(6, 6)
            .with_finish_message("Serialized and written to file")
            .build();
        spinner.enable_background();
        database::serialize(output.as_str(), &links, &lookup);
        spinner.finish();
    }
}
