use crate::r#where::{TWhere, Where};
use crate::returning::{Returning, TReturning};
use crate::sql_builder::{Sql, SqlBuilder};
use crate::struct_sql_table::StructSqlTable;

/// postgres
///
/// https://www.postgresql.org/docs/current/sql-delete.html
pub struct Delete<
    'a,
    TABLE: StructSqlTable,
> {
    pub table: TABLE,
    pub r#where: Option<Where<'a, TABLE::FIELD>>,
    pub returning: Option<Returning<TABLE::FIELD, >>,
}

impl<'a, TABLE: StructSqlTable,> Delete<'a, TABLE> {
    pub fn sql_command(self) -> Sql<'a> {
        let mut b = SqlBuilder::default();
        b.write_sql("delete from ");

        self.table.struct_sql_table(&mut b);

        if let Some(v) = self.r#where {
            v.r#where(&mut b)
        }

        if let Some(v) = self.returning {
            v.returning(&mut b)
        }

        b.sql_command()
    }
}
