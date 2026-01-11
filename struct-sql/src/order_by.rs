use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlBuilder};

pub type OrderBy<'a, COLUMN> = Vec<OrderByItem<'a, COLUMN>>;

pub trait TOrderBy<'a> {
    fn order_by(&self, sql_builder: &mut SqlBuilder<'a>);
}

impl<'a, COLUMN: Column> TOrderBy<'a> for OrderBy<'a, COLUMN> {
    fn order_by(&self, sql_builder: &mut SqlBuilder<'a>) {
        sql_builder.write_sql(" order by ");
        for (i, order_by_item) in self.iter().enumerate() {
            if i != 0 {
                sql_builder.write_sql(", ")
            }
            order_by_item.order_by_item(sql_builder);
        }
    }
}

#[derive(Clone)]
pub enum OrderByItem<'a, COLUMN: Column> {
    ASC(COLUMN),
    DESC(COLUMN),
    STMakePoint(COLUMN, SqlArg<'a>, SqlArg<'a>),
}

impl<'a, COLUMN: Column> OrderByItem<'a, COLUMN> {
    pub fn order_by_item(&self, sql_builder: &mut SqlBuilder<'a>) {
        // let column: &'a dyn Column;
        // let str: &str;

        match self {
            OrderByItem::ASC(v) => {
                v.column(sql_builder);
                sql_builder.write_sql(" ");
                sql_builder.write_sql("asc");
            }
            OrderByItem::DESC(v) => {
                v.column(sql_builder);
                sql_builder.write_sql(" ");
                sql_builder.write_sql("desc");
            }
            OrderByItem::STMakePoint(column, arg1, arg2) => {
                column.column(sql_builder);
                sql_builder.write_sql(" <-> ");
                sql_builder.write_sql("ST_MakePoint(");
                sql_builder.push_arg(*arg1);
                sql_builder.write_sql(", ");
                sql_builder.push_arg(*arg2);
                sql_builder.write_sql(")");
            }
        };
    }
}
