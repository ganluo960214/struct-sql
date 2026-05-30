use crate::column::{ColumnVec, TColumns};
use crate::deadlock::TLockPlanSql;
use crate::r#for::For;
use crate::group_by::{GroupBy, TGroupBy};
use crate::having::{Having, THaving};
use crate::limit::{Limit, TLimit};
use crate::offset::{Offset, TOffset};
use crate::order_by::{OrderBy, TOrderBy};
use crate::sql_builder::{Sql, SqlBuilder};
use crate::struct_sql_table::StructSqlTable;
use crate::r#where::TWhere;

///
/// postgres
///
/// https://www.postgresql.org/docs/current/sql-select.html
///
#[derive(Clone)]
pub struct Select<'a, TABLE: StructSqlTable, TW: TWhere<'a>> {
    pub columns:  ColumnVec<TABLE::FIELD>,
    pub from:     TABLE,
    pub r#where:  TW,
    pub group_by: Option<GroupBy<TABLE::FIELD>>,
    pub having:   Option<Having<'a, TABLE::FIELD>>,
    pub order_by: Option<OrderBy<'a, TABLE::FIELD>>,
    pub limit:    Option<Limit>,
    pub offset:   Option<Offset>,
    pub r#for:    Option<For>,
}

impl<'a, FROM: StructSqlTable + Clone, TW: TWhere<'a> + Clone> Select<'a, FROM, TW> {
    pub fn sql_command(self) -> Sql<'a> {
        let mut b = SqlBuilder::default();

        b.write_sql("select ");
        self.columns.columns(&mut b);
        b.write_sql(" from ");
        self.from.struct_sql_table(&mut b);

        self.r#where.r#where(&mut b);

        if let Some(v) = self.group_by {
            v.group_by(&mut b);
        }

        if let Some(v) = self.having {
            v.having(&mut b);
        }

        if let Some(v) = self.order_by {
            v.order_by(&mut b);
        }
        if let Some(v) = self.limit {
            v.limit(&mut b);
        }
        if let Some(v) = self.offset {
            v.offset(&mut b);
        }
        if let Some(v) = self.r#for {
            v.r#for(&mut b);
        }

        b.sql_command()
    }
}

impl<'a, TABLE, TW> TLockPlanSql<'a> for Select<'a, TABLE, TW>
where
    TABLE: StructSqlTable + Clone,
    TW: TWhere<'a> + Clone,
{
    fn to_lock_query(&self) -> (String, Sql<'a>, fn(&tokio_postgres::Row) -> String) {
        let mut table_builder = SqlBuilder::default();
        self.from.struct_sql_table(&mut table_builder);
        let table_name = table_builder.sql_command().0;

        let primary_keys = self.from.primary_key();

        let lock_select = Select {
            columns:  primary_keys,
            from:     self.from.clone(),
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
