////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Table Component
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDef {
    pub column: String,
    pub class: Option<String>,
    pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct TableDef {
    pub title : String,
    pub search_url: Option<String>,
    pub refresh_url: Option<String>,
    pub columns: Box<Vec<ColumnDef>>,
}

impl TableDef {
    pub fn new(title : &str) -> TableDef {
        TableDef {
            title : String::from(title),
            search_url: None,
            refresh_url: None,
            columns: Box::new(Vec::from([])),
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
}
