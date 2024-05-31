use clap::{Parser, Subcommand};
use log::info;

use std::{thread::sleep, time::Duration};

use crate::{
    lookup::{PageLookup, PageLookupResult},
    remap::RemappedLinks,
};

mod links;
mod pages;
mod redirects;

mod common;
mod lookup;
mod remap;
mod serialize;
mod shortest_path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    threads: Option<i32>,

    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    DeriveDB {
        /// Path to the page.sql file
        #[arg(short, long)]
        pages_sql: String,

        /// Path to the redirects.sql file
        #[arg(short, long)]
        redirects_sql: String,

        /// Path to the links.sql file
        #[arg(short, long)]
        links_sql: String,

        /// Output Path
        #[arg(short, long)]
        output: String,
    },

    Test {
        #[arg(short, long)]
        db: String,
    },
}

type ResolvedLink = (i32, i32);

fn derive_db_command(config: SubCommands, threads: i32) {
    let (pages_sql, redirects_sql, links_sql, output) = match config {
        SubCommands::DeriveDB {
            pages_sql,
            redirects_sql,
            links_sql,
            output,
        } => (pages_sql, redirects_sql, links_sql, output),
        _ => unreachable!(),
    };

    info!("Acquiring pages and redirects...");

    let (pages, redirects) = {
        let pages = pages::read_and_parse_pages(pages_sql, threads);
        let redirects = redirects::read_and_parse_redirects(redirects_sql, threads);

        (pages, redirects)
    };

    info!(
        "Got {} pages and {} redirects",
        pages.len(),
        redirects.len()
    );

    info!("Optimizing Page map");
    let lookup = lookup::PageLookup::new(pages, redirects);

    info!("Acquiring links...");
    let links = links::read_and_parse_links2(links_sql.as_str(), threads, &lookup);

    println!(
        "Got {} links (cap: {}, takes up {} kb)",
        links.len(),
        links.capacity(),
        links.capacity() * std::mem::size_of::<ResolvedLink>() / 1000
    );

    info!("Optimizing Link Map");
    let links = RemappedLinks::new(links);

    info!("Serializing and writing to file...");

    serialize::serialize(output.as_str(), &links, &lookup);

    sleep(Duration::from_secs(7));
}

fn test_command(config: SubCommands) {
    let db = match config {
        SubCommands::Test { db } => db,
        _ => unreachable!(),
    };

    println!("Loading db...");
    let data = serialize::deserialize(&db);

    let links = data.links;
    let lookup = data.pages;

    fn page_input_loop<'a>(prompt: &str, pages: &'a PageLookup) -> Option<PageLookupResult> {
        loop {
            let input = inquire::Text::new(prompt).prompt();
            if input.is_err() {
                return None;
            }

            let input = input.unwrap().replace(" ", "_");
            let page = pages.resolve_by_title(&input);
            if page.is_none() {
                println!("Page not found");
                continue;
            }

            return page;
        }
    }

    loop {
        let start = page_input_loop("Enter a page name", &lookup);
        if start.is_none() {
            break;
        }

        let end = page_input_loop("Enter a target page name", &lookup);
        if end.is_none() {
            break;
        }

        let start = start.unwrap();
        let end = end.unwrap();

        let path = shortest_path::find_shortest_path(start.id, end.id, &links);
        if path.is_none() {
            println!("No path found");
            continue;
        } else {
            println!("Path found");
            for page in path.unwrap() {
                let page = lookup.id_to_name(page).unwrap();
                println!("\t{}", page);
            }
        }
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Some(SubCommands::DeriveDB {
            pages_sql,
            redirects_sql,
            links_sql,
            output,
        }) => derive_db_command(
            SubCommands::DeriveDB {
                pages_sql,
                redirects_sql,
                links_sql,
                output,
            },
            args.threads.unwrap_or(2),
        ),
        Some(SubCommands::Test { db }) => {
            test_command(SubCommands::Test { db });
        }
        None => {
            println!("No command specified");
        }
    }
}
