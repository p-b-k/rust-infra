////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Table Component
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDef {
    pub column: String,
    pub class: Option<String>,
    pub text: String,
    pub width: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct TableDef {
    pub title: String,
    pub search_url: Option<String>,
    pub refresh_url: Option<String>,
    pub columns: Box<Vec<ColumnDef>>,
    pub action: Option<String>,
}

impl TableDef {
    pub fn new(title: &str) -> TableDef {
        TableDef {
            title: String::from(title),
            search_url: None,
            refresh_url: None,
            columns: Box::new(Vec::from([])),
            action: None,
        }
    }

    pub fn set_search_url(&mut self, url: &str) -> &TableDef {
        self.search_url = Some(String::from(url));
        self
    }

    pub fn set_refresh_url(&mut self, url: &str) -> &TableDef {
        self.refresh_url = Some(String::from(url));
        self
    }

    pub fn add_column(&mut self, cdef: ColumnDef) -> &TableDef {
        self.columns.push(cdef);
        self
    }

    pub fn set_action(&mut self, action: String) -> &TableDef {
        match &self.action {
            Some(old) => {
                warn!("Setting previously set value from {} to {action}", old)
            }
            _ => {}
        }
        self.action = Some(action);

        self
    }
}
