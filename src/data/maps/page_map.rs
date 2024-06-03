use crate::{
    data::{pages::Page, redirects::Redirect},
    indication::ProgressBuilder,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize)]
pub struct PageMap {
    // id -> name
    id_to_name: HashMap<i32, String>,
    // name -> id
    name_to_id: HashMap<String, i32>,
    // id -> redirect_id (if page is a redirect)
    id_to_redirect: HashMap<i32, i32>,
}

#[derive(Debug, PartialEq)]
pub struct PageMapResult {
    pub id: i32,
    pub title: String,
    pub redirect: Option<i32>,
}

impl PageMap {
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

    pub fn lookup_title(&self, title: &str) -> Option<PageMapResult> {
        let id = self.name_to_id(title)?;
        let redirect = self.id_to_redirect(id);
        Some(PageMapResult {
            id,
            title: title.to_string(),
            redirect,
        })
    }

    pub fn lookup_id(&self, id: i32) -> Option<PageMapResult> {
        let title = self.id_to_name(id)?.to_string();
        let redirect = self.id_to_redirect(id);
        Some(PageMapResult {
            id,
            title,
            redirect,
        })
    }

    pub fn resolve_by_title(&self, title: &str) -> Option<PageMapResult> {
        let mut page = self.lookup_title(title)?;
        while let Some(redirect) = page.redirect {
            page = self.lookup_id(redirect)?;
        }
        Some(page)
    }
}

#[test]
fn new_page_map() {
    let pages = {
        let pages = vec![
            Page {
                id: 1,
                title: "Page 1".to_string(),
                redirect: false,
            },
            Page {
                id: 2,
                title: "Page 2".to_string(),
                redirect: false,
            },
            Page {
                id: 3,
                title: "Also Page 2".to_string(),
                redirect: true,
            },
        ];
        VecDeque::from(pages)
    };

    let redirects = {
        let redirects = vec![Redirect {
            id: 3,
            title: "Page 2".to_string(),
        }];
        VecDeque::from(redirects)
    };

    let map = PageMap::new_with_progress(pages, redirects, ProgressBuilder::empty());

    assert_eq!(map.name_to_id("Page 1"), Some(1));
    assert_eq!(map.name_to_id("Page 2"), Some(2));
    assert_eq!(map.name_to_id("Also Page 2"), Some(3));

    assert_eq!(map.id_to_name(1), Some("Page 1"));
    assert_eq!(map.id_to_name(2), Some("Page 2"));
    assert_eq!(map.id_to_name(3), Some("Also Page 2"));

    assert_eq!(map.id_to_redirect(1), None);
    assert_eq!(map.id_to_redirect(2), None);
    assert_eq!(map.id_to_redirect(3), Some(2));

    assert_eq!(
        map.lookup_title("Page 1"),
        Some(PageMapResult {
            id: 1,
            title: "Page 1".to_string(),
            redirect: None
        })
    );
    assert_eq!(
        map.lookup_title("Page 2"),
        Some(PageMapResult {
            id: 2,
            title: "Page 2".to_string(),
            redirect: None
        })
    );
    assert_eq!(
        map.lookup_title("Also Page 2"),
        Some(PageMapResult {
            id: 3,
            title: "Also Page 2".to_string(),
            redirect: Some(2)
        })
    );

    assert_eq!(
        map.lookup_id(1),
        Some(PageMapResult {
            id: 1,
            title: "Page 1".to_string(),
            redirect: None
        })
    );
    assert_eq!(
        map.lookup_id(2),
        Some(PageMapResult {
            id: 2,
            title: "Page 2".to_string(),
            redirect: None
        })
    );
    assert_eq!(
        map.lookup_id(3),
        Some(PageMapResult {
            id: 3,
            title: "Also Page 2".to_string(),
            redirect: Some(2)
        })
    );

    assert_eq!(
        map.resolve_by_title("Page 1"),
        Some(PageMapResult {
            id: 1,
            title: "Page 1".to_string(),
            redirect: None
        })
    );

    assert_eq!(
        map.resolve_by_title("Page 2"),
        Some(PageMapResult {
            id: 2,
            title: "Page 2".to_string(),
            redirect: None
        })
    );

    assert_eq!(
        map.resolve_by_title("Also Page 2"),
        Some(PageMapResult {
            id: 2,
            title: "Page 2".to_string(),
            redirect: None
        })
    );
}
