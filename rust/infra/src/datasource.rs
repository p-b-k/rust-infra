////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::PooledConn;
use mysql::prelude::{FromRow, Queryable};

use log::debug;

pub struct DS
{
    pub table: String,
    pub fields: String,
}

impl DS
{
    pub fn get<T>(&self, conn: &mut PooledConn, pkey: u64) -> Result<T, String> where T : FromRow, T: Clone {
        let table = &self.table;
        let fields = &self.fields;
        let query = format!("SELECT pkey, {fields} FROM {table} WHERE pkey = {pkey}");
        debug!(target : "get", "QUERY: {query}");
        let res = conn.query_map(query, |x : T| { x });
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

    pub fn join<T>(&self, conn: &mut PooledConn, pkey: u64, fkey: &String) -> Result<Vec<T>, String>  where T : FromRow, T: Clone  {
        let table = self.table.clone();
        let fields = &self.fields;
        let query = format!("SELECT pkey, {fields} FROM {table} WHERE {fkey} = {pkey}");
        debug!(target : "join", "QUERY: {query}");
        let res = conn.query_map(query, |x : T| { x });
        match res {
            Ok(product_vers) => Ok(product_vers),
            Err(err) => Err(format!(
                "Error retrieving {table} {pkey}: {}",
                err.to_string()
            )),
        }
    }
}
