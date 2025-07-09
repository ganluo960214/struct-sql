use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;
use crate::column::Column;

pub trait StructSqlTable: Debug + Sync {
    type FIELD:Column+Clone;
    fn struct_sql_table(&self, builder: &mut SqlBuilder);
}
