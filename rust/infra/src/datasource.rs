////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::schema::{FieldDef, TableDef};

use mysql::PooledConn;

use mysql::prelude::{FromRow, Queryable};

use std::marker::PhantomData;

use log::info;

pub struct DS<T>
where
    T: FromRow,
    T: Clone,
{
    phantom: PhantomData<T>,
    pub table: String,
    pub fields: String,
}

impl<T> DS<T>
where
    T: FromRow,
    T: Clone,
{
    pub fn new(name: &str, fields: &str) -> DS<T> {
        let phantom: PhantomData<T> = PhantomData {};
        DS {
            phantom,
            table: String::from(name),
            fields: String::from(fields),
        }
    }

    pub fn from(table_def: &TableDef) -> DS<T> {
        let phantom = PhantomData {};
        let table = table_def.name.clone();
        let mut fields = String::new();

        for field in table_def.fields() {
            match field {
                FieldDef::PKey => {
                    // Do Nothing
                }
                field => {
                    fields.push_str(", ");
                    fields.push_str(field.name());
                }
            }
        }

        info!(target: "DS::from", "fields for table {table} = {fields}");

        DS {
            phantom,
            table,
            fields,
        }
    }

    pub fn get(&self, conn: &mut PooledConn, pkey: u64) -> Result<T, String>
    where
        T: FromRow,
        T: Clone,
    {
        let table = &self.table;
        let fields = &self.fields;
        let query = format!("SELECT pkey{fields} FROM {table} WHERE pkey = {pkey}");
        info!(target : "get", "QUERY: {query}");
        let res = conn.query_map(query, |x: T| x);
        match res {
            Ok(vec) => match vec.len() {
                0 => Err(String::from("No record found")),
                1 => Ok(vec[0].clone()),
                _ => Err(String::from("Multiple Records Found")),
            },
            Err(err) => Err(format!(
                "Error retrieving {table} {pkey}: {}",
                err.to_string()
            )),
        }
    }

    pub fn join(&self, conn: &mut PooledConn, pkey: u64, fkey: &String) -> Result<Vec<T>, String>
    where
        T: FromRow,
        T: Clone,
    {
        let table = self.table.clone();
        let fields = &self.fields;
        let query = format!("SELECT pkey{fields} FROM {table} WHERE {fkey} = {pkey}");
        info!(target : "join", "QUERY: {query}");
        let res = conn.query_map(query, |x: T| x);
        match res {
            Ok(product_vers) => Ok(product_vers),
            Err(err) => Err(format!(
                "Error retrieving {table} {pkey}: {}",
                err.to_string()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_get() {}
}
