use crate::column::Column;
use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;
use std::hash::Hash;
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Composite<COLUMN1: Column, COLUMN2: Column>(pub COLUMN1, pub COLUMN2);

impl<COLUMN1: Column, COLUMN2: Column> Column for Composite<COLUMN1, COLUMN2> {
    fn column(&self, builder: &mut SqlBuilder) {
        self.0.column(builder);
        builder.write_sql(".");
        self.1.column(builder);
    }

    fn is_indexed(&self) -> bool {
        false
    }
}
