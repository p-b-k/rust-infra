////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 7] = [
    FieldSpec {
        name: "fkey_svc",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "maj_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "min_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "rel_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "bld_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "bld_tag",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "schema_def",
        default: None,
        type_def: TypeDef::Data(DataType::Clob),
        nullable: true,
        unique: false,
    },
];

pub const SERVICE_VERSION: TableDef = TableDef {
    name: "service_ver",
    fields: &FIELDS,
};
