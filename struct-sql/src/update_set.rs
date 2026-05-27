use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlBuilder};
use std::collections::HashMap;

pub trait TUpdateSet<'a> {
    fn update_set(&self, sql_builder: &mut SqlBuilder<'a>);
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateValue<'a> {
    Value(SqlArg<'a>),
    Increment(SqlArg<'a>),
    Decrement(SqlArg<'a>),
}

impl<'a> From<SqlArg<'a>> for UpdateValue<'a> {
    fn from(arg: SqlArg<'a>) -> Self {
        UpdateValue::Value(arg)
    }
}

pub type UpdateSet<'a, COLUMN> = HashMap<COLUMN, UpdateValue<'a>>;

impl<'a, COLUMN: Column> TUpdateSet<'a> for UpdateSet<'a, COLUMN> {
    fn update_set(&self, builder: &mut SqlBuilder<'a>) {
        if self.is_empty() {
            return;
        }
        builder.write_sql(" set ");
        self.iter().enumerate().for_each(|(i, (column, value))| {
            if i != 0 {
                builder.write_sql(", ");
            }
            column.column(builder);
            match value {
                UpdateValue::Value(arg) => {
                    builder.write_sql(" = ");
                    builder.push_arg(*arg);
                }
                UpdateValue::Increment(arg) => {
                    builder.write_sql(" += ");
                    builder.push_arg(*arg);
                }
                UpdateValue::Decrement(arg) => {
                    builder.write_sql(" -= ");
                    builder.push_arg(*arg);
                }
            }
        })
    }
}
