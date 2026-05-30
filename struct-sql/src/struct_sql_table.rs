use crate::column::Column;
use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;

pub trait StructSqlTable: Debug + Sync {
    type FIELD: Column + Clone;
    fn struct_sql_table(&self, builder: &mut SqlBuilder);

    fn primary_key(&self) -> Vec<Self::FIELD>;
    fn primary_key_values_to_string_from_row(row: &tokio_postgres::Row) -> String;
    fn primary_key_values_to_string_from_rows(rows: &[tokio_postgres::Row]) -> Vec<String> {
        rows.iter()
            .map(|row| Self::primary_key_values_to_string_from_row(row))
            .collect()
    }
}
