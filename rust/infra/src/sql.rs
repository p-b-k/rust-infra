////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// SQL Data Structures
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{schema::TableDef, svc_schema::SchemaDef, version::Version};

pub trait AsSql {
    fn as_sql(&self) -> String;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TableRef<'a> {
    pub table: &'a TableDef,
    pub id: Option<String>,
}

impl<'a> TableRef<'a> {
    pub fn as_ref(&self) -> String {
        match &self.id {
            None => {
                let t1 = &self.table;
                String::from(t1.name)
            }
            Some(s) => s.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FieldId<'a> {
    pub field: String,
    pub table: TableRef<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SqlValue<'a> {
    Field(FieldId<'a>),
    Int(i64),
    Id(u64),
    ShortU(u32),
    String(String),
    // Timestamp(Time)
    Boolean(bool),
    Nullable(Option<Box<SqlValue<'a>>>),
    Version(Version),
    Schema(SchemaDef), // FIXME: Should try and make this a generic serialize/desrialize object
}

pub fn sql_escape(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        result.push(c);
        if c == '\'' {
            result.push(c);
        }
    }

    result
}

impl<'a> AsSql for SqlValue<'a> {
    fn as_sql(&self) -> String {
        match self {
            SqlValue::Field(fid) => {
                let table_name = &fid.table.as_ref();
                let field_name = &fid.field;
                format!("{table_name}.{field_name}")
            }
            SqlValue::String(s) => format!("'{}'", sql_escape(s)),
            SqlValue::Int(i) => format!("{i}"),
            SqlValue::Id(i) => format!("{i}"),
            SqlValue::ShortU(i) => format!("{i}"),
            SqlValue::Boolean(b) => {
                if *b {
                    String::from("'Y'")
                } else {
                    String::from("'N'")
                }
            }
            SqlValue::Nullable(o) => match o {
                None => String::from("NULL"),
                Some(v) => v.as_sql(),
            },
            SqlValue::Version(v) => format!("'{}'", v.to_sort_string()),
            SqlValue::Schema(s) => format!(
                "'{}'",
                sql_escape(
                    serde_json::to_string(&s)
                        .expect("Unable to serialize schema")
                        .as_str()
                )
            ),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SqlFilter<'a> {
    True,
    False,
    Not(Box<SqlFilter<'a>>),
    And(Box<SqlFilter<'a>>, Box<SqlFilter<'a>>),
    Or(Box<SqlFilter<'a>>, Box<SqlFilter<'a>>),
    Eq(Box<SqlValue<'a>>, Box<SqlValue<'a>>),
    Gt(Box<SqlValue<'a>>, Box<SqlValue<'a>>),
    Lt(Box<SqlValue<'a>>, Box<SqlValue<'a>>),
}

impl<'a> AsSql for SqlFilter<'a> {
    fn as_sql(&self) -> String {
        match self {
            SqlFilter::True => String::from("1 = 1"),
            SqlFilter::False => String::from("1 <> 1"),
            SqlFilter::Not(f) => format!("NOT ({})", f.as_sql()),
            SqlFilter::And(f1, f2) => format!("(({}) AND ({}))", f1.as_sql(), f2.as_sql()),
            SqlFilter::Or(f1, f2) => format!("(({}) OR ({}))", f1.as_sql(), f2.as_sql()),
            SqlFilter::Eq(v1, v2) => format!("{} = {}", v1.as_sql(), v2.as_sql()),
            SqlFilter::Gt(v1, v2) => format!("{} > {}", v1.as_sql(), v2.as_sql()),
            SqlFilter::Lt(v1, v2) => format!("{} < {}", v1.as_sql(), v2.as_sql()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::FieldSpec;

    use super::*;

    const FIELDS: [&FieldSpec; 0] = [];

    #[test]
    fn test_value() {
        let tdef = TableDef {
            name: "test",
            fields: &FIELDS,
        };
        assert_eq!(
            SqlValue::Field(FieldId {
                field: String::from("fld"),
                table: TableRef {
                    table: &tdef,
                    id: None
                }
            })
            .as_sql(),
            String::from("test.fld"),
        );
        assert_eq!(SqlValue::Int(42).as_sql(), "42");
        assert_eq!(
            SqlValue::String(String::from("hello world")).as_sql(),
            String::from("'hello world'")
        );
        assert_eq!(
            SqlValue::String(String::from("cathy's clown")).as_sql(),
            String::from("'cathy''s clown'")
        );
    }

    #[test]
    fn test_filter() {
        let tdef = TableDef {
            name: "test",
            fields: &FIELDS,
        };

        // Basics
        assert_eq!(SqlFilter::True.as_sql(), String::from("1 = 1"));
        assert_eq!(SqlFilter::False.as_sql(), String::from("1 <> 1"));
        assert_eq!(
            SqlFilter::Not(Box::new(SqlFilter::True)).as_sql(),
            String::from("NOT (1 = 1)")
        );
        assert_eq!(
            SqlFilter::And(Box::new(SqlFilter::True), Box::new(SqlFilter::False)).as_sql(),
            "((1 = 1) AND (1 <> 1))"
        );
        assert_eq!(
            SqlFilter::Or(Box::new(SqlFilter::True), Box::new(SqlFilter::False)).as_sql(),
            "((1 = 1) OR (1 <> 1))"
        );
        assert_eq!(
            SqlFilter::Eq(
                Box::new(SqlValue::Field(FieldId {
                    field: String::from("fld"),
                    table: TableRef {
                        table: &tdef,
                        id: None
                    }
                })),
                Box::new(SqlValue::Int(48))
            )
            .as_sql(),
            String::from("test.fld = 48")
        );
        assert_eq!(
            SqlFilter::Eq(
                Box::new(SqlValue::Field(FieldId {
                    field: String::from("fld"),
                    table: TableRef {
                        table: &tdef,
                        id: None
                    }
                })),
                Box::new(SqlValue::String(String::from("who'se buddy's buddy?")))
            )
            .as_sql(),
            String::from("test.fld = 'who''se buddy''s buddy?'")
        );
    }

    #[test]
    fn test_sql_escape() {
        assert_eq!(sql_escape("testing"), String::from("testing"));
        assert_eq!(
            sql_escape("Bobby'; Drop Table Students;"),
            String::from("Bobby''; Drop Table Students;")
        );
    }
}

pub mod select {
    use super::*;
    use crate::schema::TableDef;

    pub struct SqlSelect<'a> {
        pub tref: TableRef<'a>,
        pub fields: Option<Vec<SqlValue<'a>>>,
    }

    impl<'a> AsSql for SqlSelect<'a> {
        fn as_sql(&self) -> String {
            let table_id_spec = match &self.tref.id {
                None => {
                    let tref: &TableDef = self.tref.table;
                    String::from(tref.name)
                }
                Some(id) => {
                    let table_name = &self.tref.table.name;
                    format!("{table_name} {id}")
                }
            };

            let fields = match &self.fields {
                None => String::from("*"),
                Some(vec) => {
                    let mut fields = String::new();
                    let mut sep = "";

                    for f in vec {
                        fields.push_str(sep);
                        fields.push_str(f.as_sql().as_str());
                        sep = ", ";
                    }

                    fields
                }
            };
            format!("SELECT {fields} FROM {table_id_spec}")
        }
    }

    #[cfg(test)]
    mod test {
        use crate::schema::FieldSpec;

        use super::*;

        const FIELDS: [&FieldSpec; 0] = [];

        #[test]
        fn test_select() {
            let tdef = TableDef {
                name: "test",
                fields: &FIELDS,
            };

            let sel1 = SqlSelect {
                tref: TableRef {
                    table: &tdef,
                    id: None,
                },
                fields: None,
            };
            assert_eq!(sel1.as_sql(), String::from("SELECT * FROM test"));

            let tref = TableRef {
                table: &tdef,
                id: Some(String::from("t1")),
            };

            let sel2 = SqlSelect {
                fields: Some(Vec::from([
                    SqlValue::Field(FieldId {
                        table: tref.clone(),
                        field: String::from("fld1"),
                    }),
                    SqlValue::Field(FieldId {
                        table: tref.clone(),
                        field: String::from("fld2"),
                    }),
                    SqlValue::Field(FieldId {
                        table: tref.clone(),
                        field: String::from("fld3"),
                    }),
                    SqlValue::Field(FieldId {
                        table: tref.clone(),
                        field: String::from("fld4"),
                    }),
                ])),
                tref,
            };
            assert_eq!(
                sel2.as_sql(),
                String::from("SELECT t1.fld1, t1.fld2, t1.fld3, t1.fld4 FROM test t1")
            );
        }
    }
}
