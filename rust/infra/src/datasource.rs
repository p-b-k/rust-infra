////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::schema::TableDef;

use mysql::PooledConn;

use mysql::prelude::{FromRow, Queryable};

use std::marker::PhantomData;

use log::{error, info};

// #[derive(Clone, PartialEq, Eq)]
// pub struct DO<T>
// where
//     T: FromRow,
//     T: Clone,
// {
//     pub pkey: Option<u64>,
//     pub data: T,
// }

// impl<T> FromRow for DO<T>
// where
//     T: FromRow,
//     T: Clone,
// {
//     fn from_row_opt(row: Row) -> Result<Self, FromRowError> {}
// }

pub struct DS<T>
where
    T: FromRow,
    T: Clone,
{
    phantom: PhantomData<T>,
    pub table: &'static str,
    pub fields: String,
}

impl<T> DS<T>
where
    T: FromRow,
    T: Clone,
{
    // Construct a new DS
    pub fn new(name: &'static str, fields: &str) -> DS<T> {
        let phantom: PhantomData<T> = PhantomData {};
        DS {
            phantom,
            table: name,
            fields: String::from(fields),
        }
    }

    // Construct a new DS from a table
    pub fn from(table_def: &TableDef) -> DS<T> {
        let phantom = PhantomData {};
        let table = table_def.name;
        let mut fields = String::new();

        for field in table_def.fields() {
            fields.push_str(", ");
            fields.push_str(field.name);
        }

        info!(target: "DS::from", "fields for table {table} = {fields}");

        DS {
            phantom,
            table,
            fields,
        }
    }

    // Get a single object, by it's pkey
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

    // Get a collection of objects that have the same fkey value
    pub fn join(&self, conn: &mut PooledConn, pkey: u64, fkey: &String) -> Result<Vec<T>, String>
    where
        T: FromRow,
        T: Clone,
    {
        let table = self.table;
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

    pub fn all(&self, conn: &mut PooledConn) -> Vec<T> {
        let mut results_vec = Vec::new();

        let table = self.table;
        let fields = &self.fields;
        let query = format!("SELECT pkey{fields} FROM {table}");
        info!(target : "join", "QUERY: {query}");
        let res = conn.query_map(query, |x: T| x);
        match res {
            Ok(res) => {
                for r in res {
                    results_vec.push(r.clone());
                }
            }
            Err(err) => error!("Error retrieving data from {table}: {}", err.to_string()),
        }

        results_vec
    }

    // pub fn sync(&self, conn: &mut PooledConn, obj: &T) -> T {}
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_get() {}
}
