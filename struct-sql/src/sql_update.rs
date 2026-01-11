use crate::returning::{Returning, TReturning};
use crate::sql_builder::{Sql, SqlBuilder};
use crate::struct_sql_table::StructSqlTable;
use crate::update_set::{TUpdateValue, UpdateSet};
use crate::r#where::{TWhere, Where};

/// postgres
///
/// https://www.postgresql.org/docs/current/sql-update.html
pub struct Update<'a, TABLE: StructSqlTable> {
    pub table: TABLE,
    pub r#where: Option<Where<'a, TABLE::FIELD>>,
    pub set: UpdateSet<'a, TABLE::FIELD>,
    pub returning: Option<Returning<TABLE::FIELD>>,
}

impl<'a, TABLE: StructSqlTable> Update<'a, TABLE> {
    pub fn sql_command(self) -> Sql<'a> {
        let mut b = SqlBuilder::default();
        b.write_sql("update ");

        self.table.struct_sql_table(&mut b);

        self.set.update_value(&mut b);

        if let Some(v) = self.r#where {
            v.r#where(&mut b)
        }

        if let Some(v) = self.returning {
            v.returning(&mut b)
        }

        b.sql_command()
    }
}
