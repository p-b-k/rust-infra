////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// SQL Data Structures
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::schema::TableDef;

pub trait AsSql {
    fn as_sql(&self) -> String;
}

#[derive(Debug, Eq, PartialEq)]
pub struct FieldId<'a> {
    pub field: String,
    pub table: &'a TableDef,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SqlValue<'a> {
    Field(FieldId<'a>),
    Int(i64),
    String(String),
    // Timestamp(Time)
    Boolean(bool),
}

fn sql_escape(s: &str) -> String {
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
                let table_name = &fid.table.name;
                let field_name = &fid.field;
                format!("{table_name}.{field_name}")
            }
            SqlValue::String(s) => format!("'{}'", sql_escape(s)),
            SqlValue::Int(i) => format!("{i}"),
            SqlValue::Boolean(b) => {
                if *b {
                    String::from("'Y'")
                } else {
                    String::from("'N'")
                }
            }
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
    use super::*;

    #[test]
    fn test_value() {
        let tdef = TableDef {
            name: String::from("test"),
            fields: Box::new(Vec::from([])),
        };
        assert_eq!(
            SqlValue::Field(FieldId {
                field: String::from("fld"),
                table: &tdef,
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
            name: String::from("test"),
            fields: Box::new(Vec::from([])),
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
                    table: &tdef
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
                    table: &tdef
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
        pub table: &'a TableDef,
    }

    impl<'a> AsSql for SqlSelect<'a> {
        fn as_sql(&self) -> String {
            let table_id_spec = &self.table.name;
            format!("SELECT * FROM {table_id_spec}")
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_select() {
            let tdef = TableDef {
                name: String::from("test"),
                fields: Box::new(Vec::from([])),
            };

            let sel1 = SqlSelect { table: &tdef };
            assert_eq!(sel1.as_sql(), String::from("SELECT * FROM test"));
        }
    }
}
