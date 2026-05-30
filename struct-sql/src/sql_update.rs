use crate::deadlock::TLockPlanSql;

use crate::returning::{Returning, TReturning};
use crate::sql_builder::{Sql, SqlBuilder};
use crate::sql_select::Select;
use crate::struct_sql_table::StructSqlTable;
use crate::update_set::{TUpdateSet, UpdateSet};
use crate::r#where::TWhere;

/// postgres
///
/// https://www.postgresql.org/docs/current/sql-update.html
// #[derive(Default)]
pub struct Update<'a, TABLE: StructSqlTable, TW: TWhere<'a>> {
    pub table:     TABLE,
    pub r#where:   Option<TW>,
    pub set:       UpdateSet<'a, TABLE::FIELD>,
    pub returning: Option<Returning<TABLE::FIELD>>,
}

impl<'a, TABLE: StructSqlTable, TW: TWhere<'a>> Update<'a, TABLE, TW> {
    pub fn sql_command(self) -> Sql<'a> {
        let mut b = SqlBuilder::default();
        b.write_sql("update ");

        self.table.struct_sql_table(&mut b);

        self.set.update_set(&mut b);

        if let Some(v) = self.r#where {
            v.r#where(&mut b)
        }

        if let Some(v) = self.returning {
            v.returning(&mut b)
        }

        b.sql_command()
    }
}

impl<'a, TABLE, TW> TLockPlanSql<'a> for Update<'a, TABLE, TW>
where
    TABLE: StructSqlTable + Clone,
    TW: TWhere<'a> + Clone,
{
    fn to_lock_query(&self) -> (String, Sql<'a>, fn(&tokio_postgres::Row) -> String) {
        let mut table_builder = SqlBuilder::default();
        self.table.struct_sql_table(&mut table_builder);
        let table_name = table_builder.sql_command().0;

        let primary_keys = self.table.primary_key();

        let lock_select = Select {
            columns:  primary_keys,
            from:     self.table.clone(),
            r#where:  self.r#where.clone(),
            group_by: None,
            having:   None,
            order_by: None,
            limit:    None,
            offset:   None,
            r#for:    None,
        };

        (
            table_name,
            lock_select.sql_command(),
            TABLE::primary_key_values_to_string_from_row,
        )
    }
}
