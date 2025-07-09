use crate::column::{Column, ColumnVec, TColumns};
use crate::sql_builder::SqlBuilder;

pub trait TReturning {
    fn returning(&self, builder: &mut SqlBuilder);
}

pub type Returning<COLUMN> = ColumnVec<COLUMN>;

impl<COLUMN: Column> TReturning for Returning<COLUMN> {
    fn returning(&self, builder: &mut SqlBuilder) {
        builder.write_sql(" returning ");
        self.columns(builder);
    }
}
