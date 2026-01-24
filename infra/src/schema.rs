////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create Schema Def type object
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    String(u32),
    Integer,
    Date,
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
    pub fn push_to_string(&self, out: &mut String) {
        match self {
            TypeDef::PKey => out.push_str("PRIMARY KEY"),
            TypeDef::FKey(_) => out.push_str("INTEGER"),
            TypeDef::Data(data_type) => match data_type {
                DataType::String(size) => out.push_str(format!("VARCHAR({size})").as_str()),
                DataType::Integer => out.push_str("INTEGER"),
                DataType::Date => out.push_str("DATE"),
                DataType::Clob => out.push_str("CLOB"),
                DataType::Blob => out.push_str("BLOB"),
            },
        }
    }
}

impl Display for TypeDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TypeDef::PKey => write!(f, "PKEY"),
            TypeDef::FKey(table) => write!(f, "JOIN ({table})"),
            TypeDef::Data(data_type) => match data_type {
                DataType::String(size) => write!(f, "string({size})"),
                DataType::Integer => write!(f, "integer"),
                DataType::Date => write!(f, "date"),
                DataType::Clob => write!(f, "clob"),
                DataType::Blob => write!(f, "blob"),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FieldDef {
    pub name: String,
    pub type_def: TypeDef,
    pub nullable: bool,
    pub unique: bool,
}

impl Display for FieldDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} : {}", self.name, self.type_def)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TableDef {
    pub name: String,
    pub fields: Box<Vec<FieldDef>>,
}

impl Display for TableDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "TABLE:{}", self.name).unwrap();
        for field in self.fields.clone().into_iter() {
            write!(f, "\n - {}", field).unwrap();
        }
        write!(f, "")
    }
}

impl TableDef {
    pub fn create_sql(&self) -> String {
        let mut buff = String::new();
        let mut sep = " (";
        buff.push_str(format!("CREATE TABLE {}", self.name).as_str());
        for field in self.fields.clone().into_iter() {
            buff.push_str(sep);
            buff.push_str(&field.name);
            buff.push(' ');
            field.type_def.push_to_string(&mut buff);
            sep = ", ";
        }
        buff.push_str(");");
        buff
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
            println!("{}", table.create_sql())
        }
    }
}
