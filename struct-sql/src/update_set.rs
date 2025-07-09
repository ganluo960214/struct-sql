use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlBuilder};
use std::collections::HashMap;

pub trait TUpdateValue<'a> {
    fn update_value(&self, sql_builder: &mut SqlBuilder<'a>);
}

pub type UpdateSet<'a,COLUMN> = HashMap<COLUMN, SqlArg<'a>>;

impl<'a,COLUMN:Column> TUpdateValue<'a> for UpdateSet<'a,COLUMN> {
    fn update_value(&self, builder: &mut SqlBuilder<'a>) {
        builder.write_sql(" set ");
        // for (i, (column, value)) in 
        self.iter().enumerate().for_each(|(i, (column, value))|{
            if i != 0 {
                builder.write_sql(", ");
            }
            column.column(builder);
            builder.write_sql(" = ");
            builder.push_arg(*value);
        }) 
    }
}
