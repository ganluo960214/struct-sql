use crate::column::Column;
use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;

pub trait StructSqlTable: Debug + Sync {
    type FIELD: Column + Clone;
    fn struct_sql_table(&self, builder: &mut SqlBuilder);
}
