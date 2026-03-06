////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldDef; 2] = [
    FieldDef::Field(FieldSpec {
        name: "prod_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    }),
    FieldDef::Field(FieldSpec {
        name: "prod_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    }),
];

pub const PRODUCT: TableDef = TableDef {
    name: "product",
    fields: &FIELDS,
};
