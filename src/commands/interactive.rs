use clap::Args;

use crate::{
    data::{
        algorithm::bfs,
        database::Database,
        maps::page_map::{PageMap, PageMapResult},
    },
    indication::ProgressBuilder,
};

use super::ArgExecutor;

#[derive(Args, Debug)]
pub struct InteractiveArgs {
    /// Database derived using derive-db command
    #[arg(short, long)]
    db: String,
}

impl ArgExecutor for InteractiveArgs {
    fn execute(&self) {
        interactive_cmd(self);
    }
}

fn interactive_cmd(args: &InteractiveArgs) {
    let db = args.db.to_string();

    let db = {
        let spinner = ProgressBuilder::spinner()
            .with_message("📝 Deserializing DB")
            .build();
        spinner.enable_background();
        let data = Database::from_file(&db);
        spinner.finish();
        data
    };

    println!(
        "Usage: Enter a start page and a target page to find the shortest path between them
If you want to exit press ctrl+d or ctrl+c\n"
    );

    let links = db.links;
    let lookup = db.pages;

    fn page_input_loop(prompt: &str, pages: &PageMap) -> Option<PageMapResult> {
        loop {
            let input = inquire::Text::new(prompt).prompt();
            if input.is_err() {
                return None;
            }

            let input = input.unwrap().replace(' ', "_");
            let page = pages.resolve_by_title(&input);
            if page.is_none() {
                println!("Page not found");
                continue;
            }

            return page;
        }
    }

    loop {
        let start = page_input_loop("Enter a start page name", &lookup);
        if start.is_none() {
            break;
        }

        let end = page_input_loop("Enter a target page name", &lookup);
        if end.is_none() {
            break;
        }

        let start = start.unwrap();
        let end = end.unwrap();

        let (path, time) = {
            let spinner = ProgressBuilder::spinner()
                .with_message("Searching for path")
                .build();
            spinner.enable_background();

            let time_before = std::time::Instant::now();
            let path = bfs::find_shortest_path(start.id, end.id, &links);
            let time = time_before.elapsed().as_millis();

            spinner.finish();

            (path, time)
        };

        if path.is_none() {
            println!("😔 No path found");
            continue;
        } else {
            println!("🎉 Path found in {time}ms");
            for page in path.unwrap() {
                let page = lookup.id_to_name(page).unwrap();
                println!("\t{}", page);
            }
        }
    }
}
