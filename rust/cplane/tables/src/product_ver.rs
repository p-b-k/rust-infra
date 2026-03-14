////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldDef; 6] = [
    FieldDef::Field(FieldSpec {
        name: "fkey_prod",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "maj_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "min_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "rel_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "bld_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "bld_tag",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: true,
        unique: true,
    }),
];

pub const PRODUCT_VERSION: TableDef = TableDef {
    name: "product_ver",
    fields: &FIELDS,
};
