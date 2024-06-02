use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{indication::ProgressBuilder, pages::Page, redirects::Redirect};

#[derive(Serialize, Deserialize)]
pub struct PageLookup {
    // id -> name
    id_to_name: HashMap<i32, String>,
    // name -> id
    name_to_id: HashMap<String, i32>,
    // id -> redirect_id (if page is a redirect)
    id_to_redirect: HashMap<i32, i32>,
}

pub struct PageLookupResult {
    pub id: i32,
    pub title: String,
    pub redirect: Option<i32>,
}

impl PageLookup {
    fn new_internal(
        pages: VecDeque<Page>,
        redirect: VecDeque<Redirect>,
        progress: ProgressBuilder,
    ) -> Self {
        let progress = progress
            .with_len((pages.len() + redirect.len()) as u64)
            .build();

        let mut id_to_name = HashMap::new();
        let mut name_to_id = HashMap::new();
        let mut id_to_redirect = HashMap::new();

        for page in pages {
            id_to_name.insert(page.id, page.title.clone());
            name_to_id.insert(page.title.clone(), page.id);

            progress.inc(1);
        }

        for redirect in redirect {
            let from = redirect.id;
            let to = name_to_id.get(&redirect.title);
            if let Some(&to) = to {
                id_to_redirect.insert(from, to);
            }

            progress.inc(1);
        }

        progress.finish();

        Self {
            id_to_name,
            name_to_id,
            id_to_redirect,
        }
    }

    pub fn new(pages: VecDeque<Page>, redirect: VecDeque<Redirect>) -> Self {
        Self::new_internal(pages, redirect, ProgressBuilder::empty())
    }

    pub fn new_with_progress(
        pages: VecDeque<Page>,
        redirect: VecDeque<Redirect>,
        progress: ProgressBuilder,
    ) -> Self {
        Self::new_internal(pages, redirect, progress)
    }

    pub fn name_to_id(&self, name: &str) -> Option<i32> {
        self.name_to_id.get(name).copied()
    }

    pub fn id_to_name(&self, id: i32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn id_to_redirect(&self, id: i32) -> Option<i32> {
        self.id_to_redirect.get(&id).copied()
    }

    pub fn lookup_title(&self, title: &str) -> Option<PageLookupResult> {
        let id = self.name_to_id(title)?;
        let redirect = self.id_to_redirect(id);
        Some(PageLookupResult {
            id,
            title: title.to_string(),
            redirect,
        })
    }

    pub fn lookup_id(&self, id: i32) -> Option<PageLookupResult> {
        let title = self.id_to_name(id)?.to_string();
        let redirect = self.id_to_redirect(id);
        Some(PageLookupResult {
            id,
            title,
            redirect,
        })
    }

    pub fn resolve_by_title(&self, title: &str) -> Option<PageLookupResult> {
        let mut page = self.lookup_title(title)?;
        while let Some(redirect) = page.redirect {
            page = self.lookup_id(redirect)?;
        }
        Some(page)
    }
}
