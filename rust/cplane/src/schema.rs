////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::marker::PhantomData;

use log::warn;

use infra::schema::{
    DBUser, DataType, FieldDef, FieldSpec, GrantInfo, SchemaDef, TableDef, TypeDef,
};

use infra::datasource::DS;

use mysql::PooledConn;
use mysql_common::prelude::FromRow;
use serde::{Deserialize, Serialize};
use time::Time;

// ---------------------------------------------------------------------------------------------------------------------
// Create a datasource object
// ---------------------------------------------------------------------------------------------------------------------

pub fn fields_from_table(def: &TableDef) -> String {
    let mut fields = String::from("pkey");
    for field in def.fields() {
        fields.push_str(", ");
        fields.push_str(field.name());
    }

    fields
}

struct DO<'a, T>
where
    T: FromRow,
    T: Clone,
{
    def: &'a TableDef,
    phantom: PhantomData<T>,
    ds: DS,
}

impl<'a, T> DO<'a, T>
where
    T: FromRow,
    T: Clone,
{
    // I guess this needs the lifetime parameter because it is "static" (i.e. it does not refernce self) ...
    pub fn from_table(def: &'a TableDef) -> DO<'a, T> {
        let table = def.name.clone();
        let fields = fields_from_table(&def);
        DO {
            def,
            phantom: PhantomData,
            ds: DS { table, fields },
        }
    }

    // ... whereas this does not require one because it pickes it up from self
    pub fn get(&self, conn: &mut PooledConn, pkey: u64) -> Option<T> {
        match self.ds.get(conn, pkey) {
            Ok(row) => Some(row),
            Err(msg) => {
                warn!(target:"DS.get", "No object returned: {msg}");
                None
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Account {
    pub pkey: u64,
    pub acct_id: String,
    pub acct_name: String,
}

fn mk_acct() -> TableDef {
    TableDef {
        name: String::from("account"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("acct_id"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("acct_name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    pub pkey: u64,
    pub prod_id: String,
    pub prod_name: String,
}

fn mk_prod() -> TableDef {
    TableDef {
        name: String::from("product"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("prod_id"),
                default: None,
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("prod_name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub pkey: u64,
    pub fkey_prod: String,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_rel: Option<u32>,
    pub bld_tag: Option<String>,
}

fn mk_prod_ver() -> TableDef {
    TableDef {
        name: String::from("product_ver"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("maj_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("min_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("rel_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_tag"),
                default: None,
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: true,
                unique: true,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: bool,
}

fn mk_svc() -> TableDef {
    TableDef {
        name: String::from("service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("svc_id"),
                default: None,
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("svc_name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("is_global"),
                default: Some(String::from("'Y'")),
                type_def: TypeDef::Data(DataType::Boolean),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub pkey: u64,
    pub fkey_svc: String,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_rel: Option<u32>,
    pub bld_tag: Option<String>,
    pub schema_def: Option<String>,
}

fn mk_svc_ver() -> TableDef {
    TableDef {
        name: String::from("service_ver"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("maj_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("min_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("rel_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_tag"),
                default: None,
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("schema_def"),
                default: None,
                type_def: TypeDef::Data(DataType::Clob),
                nullable: true,
                unique: false,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductService {
    pub pkey: u64,
    pub fkey_prod: u64,
    pub fkey_svc: u64,
}

fn mk_prod_svc() -> TableDef {
    TableDef {
        name: String::from("product_service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_ver")),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("service_ver")),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    pub pkey: u64,
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}

fn mk_req() -> TableDef {
    TableDef {
        name: String::from("request"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("req_type"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_start"),
                default: None,
                type_def: TypeDef::Data(DataType::Timestamp),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_status"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    pub pkey: u64,
    pub fkey_acct: u64,
}

fn mk_tent() -> TableDef {
    TableDef {
        name: String::from("tenant"),
        fields: Box::new(Vec::from([FieldDef::Field(FieldSpec {
            name: String::from("fkey_acct"),
            default: None,
            type_def: TypeDef::FKey(String::from("tenant")),
            nullable: false,
            unique: false,
        })])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    pub pkey: u64,
    pub fkey_req: u64,
    pub status: String,
}

fn mk_task() -> TableDef {
    TableDef {
        name: String::from("task"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_req"),
                default: None,
                type_def: TypeDef::FKey(String::from("request")),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("status"),
                default: Some(String::from("'PENDING'")),
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    pub pkey: u64,
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}

fn mk_prod_tent() -> TableDef {
    TableDef {
        name: String::from("product_tenant"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_tnet"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_tenant")),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_ver")),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    pub pkey: u64,
    pub name: String,
    pub host: String,
    pub port: u32,
    pub status: u32,
    pub last_check: Time,
}

fn mk_worker() -> TableDef {
    TableDef {
        name: String::from("worker"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("host"),
                default: None,
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("port"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("status"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("host"),
                default: None,
                type_def: TypeDef::Data(DataType::Timestamp),
                nullable: true,
                unique: false,
            }),
        ])),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

pub fn build_schema_def() -> SchemaDef {
    let account = mk_acct();
    let product_def = mk_prod();
    let product_ver_def = mk_prod_ver();
    let service_def = mk_svc();
    let service_ver_def = mk_svc_ver();
    let product_service = mk_prod_svc();
    let request = mk_req();
    let task = mk_task();
    let tenant = mk_tent();
    let product_tenant = mk_prod_tent();

    let def = SchemaDef {
        users: Box::new(Vec::from([DBUser {
            role_id: String::from("app"),
            grants: Box::new(Vec::from([GrantInfo::All])),
        }])),
        tables: Box::new(Vec::from([
            account,
            service_def,
            service_ver_def,
            product_def,
            product_ver_def,
            product_service,
            request,
            task,
            tenant,
            product_tenant,
        ])),
    };

    def
}
