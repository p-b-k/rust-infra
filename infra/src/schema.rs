////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create Schema Def type object
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TypeDef {
    PKey,
    FKey(String),
    String(u32),
    Integer,
    Date,
    Clob,
    Blob,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FieldDef {
    pub name: String,
    pub type_def: TypeDef,
    pub nullable: bool,
    pub unique: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TableDef {
    pub name: String,
    pub fields: Box<Vec<FieldDef>>,
}

impl Display for TableDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SchemaDef {
    pub tables: Box<Vec<TableDef>>,
}

impl SchemaDef {
    pub fn display(&self) {
        println!("I am a schema!");
        for table in self.tables.clone().into_iter() {
            println!("TABLE: {table}");
        }
    }
}
