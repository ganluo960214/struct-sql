use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Column: Debug + Eq + PartialEq + Hash {
    fn column(&self, builder: &mut SqlBuilder);
    fn is_indexed(&self) -> bool;
}

pub type ColumnVec<Column> = Vec<Column>;

pub trait TColumns {
    fn columns(&self, builder: &mut SqlBuilder);
}

impl<COLUMN: Column> TColumns for ColumnVec<COLUMN> {
    fn columns(&self, builder: &mut SqlBuilder) {
        self.into_iter().enumerate().for_each(|(index, column)| {
            if index != 0 {
                builder.write_sql(", ")
            }
            column.column(builder);
        });
    }
}

pub type ColumnSlice<COLUMN,const COLUMN_N:usize> = [COLUMN;COLUMN_N];
impl<COLUMN:Column,const COLUMN_N: usize> TColumns for ColumnSlice<COLUMN,COLUMN_N> {
    fn columns(&self, builder: &mut SqlBuilder) {
        self.into_iter().enumerate().for_each(|(index, column)| {
            if index != 0 {
                builder.write_sql(", ")
            }
            column.column(builder);
        });

    }
}
