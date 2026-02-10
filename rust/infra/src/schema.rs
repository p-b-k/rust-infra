////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create Schema Def type object
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Formatter};

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
    FKey(String),
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
    pub name: String,
    pub type_def: TypeDef,
    pub nullable: bool,
    pub unique: bool,
    pub default: Option<String>,
}

impl Display for FieldSpec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} : {}", self.name, self.type_def)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FieldDef {
    PKey,
    Field(FieldSpec),
}

impl FieldDef {
    pub fn name(&self) -> &str {
        match self {
            FieldDef::PKey => "pkey",
            FieldDef::Field(field_spec) => field_spec.name.as_str(),
        }
    }

    pub fn type_def(&self) -> DataType {
        match self {
            FieldDef::PKey => DataType::Integer,
            FieldDef::Field(field_spec) => field_spec.type_def.as_data_type(),
        }
    }

    pub fn type_string(&self) -> String {
        match self {
            FieldDef::PKey => String::from("PRIMARY KEY"),
            FieldDef::Field(field_spec) => format!("{}", field_spec.type_def),
        }
    }

    pub fn print(&self) {
        let mut unique = " ";
        let mut nullable = " ";
        let name = self.name();
        let data_type = self.type_string();

        match self {
            FieldDef::PKey => {
                unique = "*";
            }
            FieldDef::Field(field_spec) => {
                if field_spec.nullable {
                    nullable = "?";
                }

                if field_spec.unique {
                    unique = "*";
                }
            }
        }

        println!(" {unique} {nullable} {name:<16} : {data_type}");
    }
}

impl Display for FieldDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FieldDef::PKey => write!(f, "pkey"),
            FieldDef::Field(field_spec) => {
                write!(f, "{} : {}", field_spec.name, field_spec.type_def)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TableDef {
    pub name: String,
    pub fields: Box<Vec<FieldDef>>,
}

impl Display for TableDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "TABLE:{}({})", self.name, self.fields.len())
    }
}

impl TableDef {
    pub fn print(&self) {
        println!("TABLE:{}", self.name);
        for field in self.fields.clone().into_iter() {
            field.print();
        }
        println!();
    }
}

pub struct FieldIter<'a> {
    table: &'a TableDef,
    index: usize,
}

impl Iterator for FieldIter<'_> {
    type Item = FieldDef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.index = self.index + 1;
            Some(FieldDef::PKey)
        } else if self.index > self.table.fields.len() {
            None
        } else {
            let field_index = self.index - 1;
            self.index = self.index + 1;
            Some(self.table.fields[field_index].clone())
        }
    }
}

impl TableDef {
    pub fn create_sql(&self) -> String {
        let mut buff = String::new();
        let mut sep = " (";
        buff.push_str(format!("CREATE TABLE {}", self.name).as_str());
        for field in self.fields() {
            buff.push_str(sep);
            match &field {
                FieldDef::PKey => buff.push_str("pkey INTEGER PRIMARY KEY"),
                FieldDef::Field(field_spec) => {
                    buff.push_str(&field_spec.name);
                    buff.push(' ');
                    field_spec.type_def.push_to_string(&mut buff);
                    if field_spec.unique {
                        buff.push_str(" UNIQUE");
                    }
                    if !field_spec.nullable {
                        buff.push_str(" NOT NULL");
                    }
                    match &field_spec.default {
                        Some(val) => {
                            buff.push_str(format!(" DEFAULT {val}").as_str());
                        }
                        None => {}
                    }
                }
            }
            sep = ", ";
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SchemaDef {
    pub tables: Box<HashMap<String, TableDef>>,
    pub users: Box<Vec<DBUser>>,
}

impl SchemaDef {
    pub fn display(&self) {
        for (_, table) in self.tables.clone().iter() {
            println!("TABLE: {table}");
            println!("{}", table.create_sql())
        }
    }
}

pub enum SqlFilter {
    True,
    False,
    And(Box<SqlFilter>, Box<SqlFilter>),
    Or(Box<SqlFilter>, Box<SqlFilter>),
}
