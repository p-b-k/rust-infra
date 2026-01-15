////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::Conn;
use std::sync::Arc;

pub trait Record<T> {
    fn from(&self) -> T;
}

pub trait Query<T> {
    fn execute(conn: &mut Conn) -> Vec<T>;
}

pub trait DataSource {
    fn record<T>(conn: &mut Conn, table: &String, pkey: i32) -> Result<T, String>;
    fn query<T>(conn: &mut Conn, name: &String) -> Result<Arc<Vec<T>>, String>;
}

pub struct StdDS {}

impl DataSource for StdDS {
    fn record<T>(_conn: &mut Conn, _table: &String, _pkey: i32) -> Result<T, String> {
        Err(String::from("StdDs::record not implemented"))
    }

    fn query<T>(_conn: &mut Conn, _name: &String) -> Result<Arc<Vec<T>>, String> {
        Err(String::from("StdDs::query not implemented"))
    }
}
