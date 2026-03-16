////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Data Object and related structures
// IS THIS USED?
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use log::debug;

use std::marker::PhantomData;

use log::debug;
use mysql::{
    Error, PooledConn, Row,
    prelude::{FromRow, Queryable},
};
use serde::Serialize;

use crate::schema::TableDef;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DObj<'a, T>
where
    T: FromRow,
    T: Clone,
    T: Serialize,
{
    pub table: &'a TableDef,
    pub pkey: Option<u64>,
    pub obj: T,
}

impl<'a, T> Serialize for DObj<'a, T>
where
    T: FromRow,
    T: Clone,
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = (self.pkey, &self.obj).serialize(serializer).unwrap();
        Ok(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DObjFactory<'a, T>
where
    T: Clone,
    T: FromRow,
{
    pub phantom: PhantomData<T>,
    pub table: &'a TableDef,
}

impl<'a, T> DObjFactory<'a, T>
where
    T: Clone,
    T: FromRow,
    T: Serialize,
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
        debug!(target : "fetch", "QUERY: {query}");
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
    ) -> Result<Vec<DObj<'a, T>>, Error>
    where
        T: FromRow,
        T: Clone,
    {
        let table = self.table.name;
        let fields = &self.fields();
        let query = format!("SELECT pkey, {fields} FROM {table} WHERE {fkey} = {pkey}");
        debug!(target : "join", "QUERY: {query}");

        conn.query_map(query, |row: Row| {
            let pkey = row.get("pkey").unwrap();
            let obj: T = T::from_row(row);
            return self.from(obj, pkey);
        })
    }

    pub fn all(&self, conn: &mut PooledConn) -> Result<Vec<DObj<'a, T>>, Error> {
        let table = self.table.name;
        let fields = &self.fields();
        let query = format!("SELECT pkey, {fields} FROM {table}");
        debug!(target : "all", "QUERY: {query}");

        conn.query_map(query, |row: Row| {
            let pkey = row.get("pkey").unwrap();
            let obj: T = T::from_row(row);
            return self.from(obj, pkey);
        })
    }
}
