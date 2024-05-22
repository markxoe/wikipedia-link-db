use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{pages::Page, redirects::Redirect};

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
    pub fn new(pages: Vec<Page>, redirect: Vec<Redirect>) -> Self {
        let mut id_to_name = HashMap::new();
        let mut name_to_id = HashMap::new();
        let mut id_to_redirect = HashMap::new();

        for page in pages {
            id_to_name.insert(page.id, page.title.clone());
            name_to_id.insert(page.title.clone(), page.id);
        }

        for redirect in redirect {
            let from = redirect.id;
            let to = name_to_id.get(&redirect.title);
            if let Some(&to) = to {
                id_to_redirect.insert(from, to);
            }
        }

        Self {
            id_to_name,
            name_to_id,
            id_to_redirect,
        }
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
}
