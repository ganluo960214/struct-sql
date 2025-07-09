use crate::column::{Column, ColumnVec, TColumns};
use crate::sql_builder::SqlBuilder;

pub trait TGroupBy {
    fn group_by(&self, builder: &mut SqlBuilder);
}

pub type GroupBy<COLUMN, > = ColumnVec<COLUMN,>;

impl<COLUMN: Column,> TGroupBy for GroupBy<COLUMN, > {
    fn group_by(&self, builder: &mut SqlBuilder) {
        builder.write_sql(" group by ");
        self.columns(builder);
    }
}
