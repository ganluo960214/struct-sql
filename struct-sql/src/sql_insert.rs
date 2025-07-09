use crate::column::{Column, TColumns};
use crate::on_conflict::OnConflict;
use crate::returning::{Returning, TReturning};
use crate::sql_builder::{Sql, SqlArg, SqlBuilder};
use crate::struct_sql_table::StructSqlTable;
///
/// Insert
///
/// postgres
///
/// https://www.postgresql.org/docs/current/sql-insert.html
pub struct Insert<
    'a,
    TABLE: StructSqlTable,
    const INSERT_VALUE_N: usize,
> {
    pub table: TABLE,
    pub insert_value: InsertValue<'a, TABLE::FIELD, INSERT_VALUE_N>,
    pub on_conflict: Option<OnConflict<'a, TABLE::FIELD>>,
    pub returning: Option<Returning<TABLE::FIELD>>,
}

impl<
    'a,
    TABLE: StructSqlTable,
    const INSERT_VALUE_N: usize,
> Insert<'a, TABLE, INSERT_VALUE_N, >
{
    pub fn sql_command(self) -> Sql<'a> {
        let mut b = SqlBuilder::default();
        b.write_sql("insert into ");

        self.table.struct_sql_table(&mut b);

        self.insert_value.insert_value(&mut b);

        if let Some(v) = &self.on_conflict {
            v.on_conflict(&mut b);
        }

        if let Some(v) = &self.returning {
            v.returning(&mut b)
        }

        b.sql_command()
    }
}

///
/// InsertValue
///
pub struct InsertValue<'a, COLUMN: Column, const COLUMN_SIZE: usize>(
    pub [COLUMN; COLUMN_SIZE],
    pub Vec<[SqlArg<'a>; COLUMN_SIZE]>,
);

impl<'a, COLUMN: Column, const COLUMN_SIZE: usize> InsertValue<'a, COLUMN, COLUMN_SIZE> {
    pub fn insert_value(&self, builder: &mut SqlBuilder<'a>) {
        builder.write_sql(" (");
        self.0.columns(builder);
        builder.write_sql(") values (");
        for value in self.1.iter() {
            builder.push_args_iter(value.iter())
        }
        builder.write_sql(")");
    }
}
