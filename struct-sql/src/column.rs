use crate::sql_builder::SqlBuilder;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Column: Debug + Eq + PartialEq + Hash {
    fn column(&self, builder: &mut SqlBuilder);
    fn is_indexed(&self) -> bool;
}

// #[derive(Debug,Eq,PartialEq,Hash)]
// pub struct DummyColumn;
// impl Column for DummyColumn {
//     fn column(&self, _builder: &mut SqlBuilder) {
//     }

//     fn is_indexed(&self) -> bool {
//         false
//     }
// }

pub type ColumnVec<Column> = Vec<Column>;

pub trait TColumns<'a> {
    fn columns(&self, builder: &mut SqlBuilder<'a>);
}

impl<'a, COLUMN: Column> TColumns<'a> for ColumnVec<COLUMN> {
    fn columns(&self, builder: &mut SqlBuilder) {
        self.iter().enumerate().for_each(|(index, column)| {
            if index != 0 {
                builder.write_sql(", ")
            }
            column.column(builder);
        });
    }
}

pub type ColumnSlice<COLUMN, const COLUMN_N: usize> = [COLUMN; COLUMN_N];
impl<'a, COLUMN: Column, const COLUMN_N: usize> TColumns<'a> for ColumnSlice<COLUMN, COLUMN_N> {
    fn columns(&self, builder: &mut SqlBuilder) {
        self.iter().enumerate().for_each(|(index, column)| {
            if index != 0 {
                builder.write_sql(", ")
            }
            column.column(builder);
        });
    }
}
