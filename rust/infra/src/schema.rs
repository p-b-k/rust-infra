////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create Schema Def type object
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Formatter};

use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    Boolean,
    String(u32),
    Integer,
    Timestamp,
    Clob,
    Blob,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TypeDef {
    PKey,
    FKey(&'static str),
    Data(DataType),
}

impl TypeDef {
    pub fn as_data_type(&self) -> DataType {
        match self {
            TypeDef::PKey => DataType::Integer,
            TypeDef::FKey(_) => DataType::Integer,
            TypeDef::Data(data_type) => data_type.clone(),
        }
    }
    pub fn push_to_string(&self, out: &mut String) {
        match self {
            TypeDef::PKey => out.push_str("INTEGER"),
            TypeDef::FKey(_) => out.push_str("INTEGER"),
            TypeDef::Data(data_type) => match data_type {
                DataType::Boolean => out.push_str("VARCHAR(1)"),
                DataType::String(size) => out.push_str(format!("VARCHAR({size})").as_str()),
                DataType::Integer => out.push_str("INTEGER"),
                DataType::Timestamp => out.push_str("DATE"),
                DataType::Clob => out.push_str("LONGTEXT"), // Using MySQL Syntax for now, TODO add RDBMS layer
                DataType::Blob => out.push_str("BLOB"),
            },
        }
    }
}

impl Display for TypeDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TypeDef::PKey => write!(f, "pkey"),
            TypeDef::FKey(table) => write!(f, "join({table})"),
            TypeDef::Data(data_type) => match data_type {
                DataType::Boolean => write!(f, "bool"),
                DataType::String(size) => write!(f, "string({size})"),
                DataType::Integer => write!(f, "integer"),
                DataType::Timestamp => write!(f, "date"),
                DataType::Clob => write!(f, "clob"),
                DataType::Blob => write!(f, "blob"),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FieldSpec {
    pub name: &'static str,
    pub type_def: TypeDef,
    pub nullable: bool,
    pub unique: bool,
    pub default: Option<&'static str>,
}

impl Display for FieldSpec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} : {}", self.name, self.type_def)
    }
}

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct TableDef {
    pub name: &'static str,
    pub fields: &'static [FieldSpec],
}

impl Display for TableDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "TABLE:{}({})", self.name, self.fields.len())
    }
}

impl TableDef {
    pub fn print(&self) {
        println!("TABLE:{}", self.name);
        for field in self.fields {
            println!("{field}");
        }
        println!();
    }
}

pub struct FieldIter<'a> {
    table: &'a TableDef,
    index: usize,
}

impl Iterator for FieldIter<'_> {
    type Item = FieldSpec;

    fn next(&mut self) -> Option<Self::Item> {
        debug!(target: "field iterator", "self.index = {}", self.index);

        if self.index < self.table.fields.len() {
            let field_index = self.index;
            self.index = self.index + 1;
            Some(self.table.fields[field_index].clone())
        } else {
            None
        }
    }
}

impl TableDef {
    pub fn create_sql(&self) -> String {
        let mut buff = String::new();
        buff.push_str(format!("CREATE TABLE {}", self.name).as_str());
        buff.push_str("(pkey INTEGER PRIMARY KEY");
        for field in self.fields() {
            buff.push_str(", ");
            buff.push_str(&field.name);
            buff.push(' ');
            field.type_def.push_to_string(&mut buff);
            if field.unique {
                buff.push_str(" UNIQUE");
            }
            if !field.nullable {
                buff.push_str(" NOT NULL");
            }
            match &field.default {
                Some(val) => {
                    buff.push_str(format!(" DEFAULT {val}").as_str());
                }
                None => {}
            }
        }
        buff.push_str(")");
        buff
    }

    pub fn fields(&self) -> FieldIter<'_> {
        FieldIter {
            table: self,
            index: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GrantInfo {
    All,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DBUser {
    pub role_id: String,
    pub grants: Box<Vec<GrantInfo>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaDef {
    pub tables: Box<HashMap<String, &'static TableDef>>,
    pub users: Box<HashMap<String, DBUser>>,
}

impl SchemaDef {
    pub fn display(&self) {
        for (_, table) in self.tables.clone().iter() {
            println!("TABLE: {table}");
            println!("{}", table.create_sql())
        }
    }
}
