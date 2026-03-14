////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Data Object and related structures
// IS THIS USED?
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use log::debug;

use std::marker::PhantomData;

use log::{error, info};
use mysql::{
    PooledConn,
    prelude::{FromRow, Queryable},
};
use serde::Serialize;

use crate::schema::TableDef;

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct DObj<'a, T>
where
    T: Clone,
{
    pub table: &'a TableDef,
    pub pkey: Option<u64>,
    pub obj: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DObjFactory<'a, T> {
    pub phantom: PhantomData<T>,
    pub table: &'a TableDef,
}

impl<'a, T> DObjFactory<'a, T>
where
    T: Clone,
    T: FromRow,
{
    pub fn new(&self, obj: T) -> DObj<'a, T> {
        DObj {
            table: self.table,
            pkey: None,
            obj: obj.clone(),
        }
    }

    pub fn from(&self, obj: T, pkey: u64) -> DObj<'a, T> {
        DObj {
            table: self.table,
            pkey: Some(pkey),
            obj: obj.clone(),
        }
    }

    fn fields(&self) -> String {
        // self.table.fields.iter().map(|it| it.name)
        let mut fields = String::new();
        let mut sep = "";

        for field in self.table.fields() {
            match field {
                field => {
                    fields.push_str(sep);
                    fields.push_str(field.name);
                    sep = ", ";
                }
            }
        }

        fields
    }

    pub fn fetch(&self, conn: &mut PooledConn, pkey: u64) -> Result<DObj<'a, T>, String>
    where
        T: FromRow,
        T: Clone,
    {
        let table = &self.table.name;
        let fields = &self.fields();
        let query = format!("SELECT {fields} FROM {table} WHERE pkey = {pkey}");
        info!(target : "fetch", "QUERY: {query}");
        let res = conn.query_map(query, |x: T| x);
        match res {
            Ok(vec) => match vec.len() {
                0 => Err(String::from("No record found")),
                1 => Ok(self.from(vec[0].clone(), pkey)),
                _ => Err(String::from("Multiple Records Found")),
            },
            Err(err) => Err(format!(
                "Error retrieving {table} {pkey}: {}",
                err.to_string()
            )),
        }
    }

    pub fn join(
        &self,
        conn: &mut PooledConn,
        pkey: u64,
        fkey: &String,
    ) -> Result<Vec<DObj<'a, T>>, String>
    where
        T: FromRow,
        T: Clone,
    {
        let table = self.table.name;
        let fields = &self.fields();
        let query = format!("SELECT pkey, {fields} FROM {table} WHERE {fkey} = {pkey}");
        info!(target : "join", "QUERY: {query}");

        let res = conn.query_map(query, |x: T| self.new(x));
        match res {
            Ok(product_vers) => Ok(product_vers),
            Err(err) => Err(format!(
                "Error retrieving {table} {pkey}: {}",
                err.to_string()
            )),
        }
    }

    pub fn all(&self, conn: &mut PooledConn) -> Vec<DObj<'a, T>> {
        let mut results_vec = Vec::new();

        let table = self.table.name;
        let fields = &self.fields();
        let query = format!("SELECT pkey, {fields} FROM {table}");
        info!(target : "all", "QUERY: {query}");

        // let res = conn.query_iter(query);

        let res = conn.query_map(query, |x: T| self.new(x));
        match res {
            Ok(res) => {
                for r in res {
                    results_vec.push(r);
                }
            }
            Err(err) => error!("Error retrieving data from {table}: {}", err.to_string()),
        }

        results_vec
    }
}
