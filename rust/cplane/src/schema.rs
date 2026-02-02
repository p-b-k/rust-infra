////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{
    DBUser, DataType, FieldDef, FieldSpec, GrantInfo, SchemaDef, TableDef, TypeDef,
};

use infra::datasource::DS;

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

trait DataSource
{
    fn as_ds() -> DS;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct AccountDO {
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
pub struct ProductDO {
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
pub struct ProductVerDO {
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
pub struct ServiceDO {
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
pub struct ServiceVerDO {
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
pub struct ProductServiceDO {
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
pub struct RequestDO {
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
pub struct TenantDO {
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
pub struct TaskDO {
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
pub struct ProductTenantDO {
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

    SchemaDef {
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
    }
}
