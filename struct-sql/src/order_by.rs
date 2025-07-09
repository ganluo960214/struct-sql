use crate::column::Column;
use crate::sql_builder::SqlBuilder;

pub type OrderBy<COLUMN> = Vec<OrderByItem<COLUMN>>;

pub trait TOrderBy {
    fn order_by(&self, sql_builder: &mut SqlBuilder);
}

impl<COLUMN: Column> TOrderBy for OrderBy<COLUMN> {
    fn order_by(&self, sql_builder: &mut SqlBuilder) {
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
pub enum OrderByItem<COLUMN: Column> {
    ASC(COLUMN),
    DESC(COLUMN),
}

impl<COLUMN: Column> OrderByItem<COLUMN> {
    pub fn order_by_item(&self, sql_builder: &mut SqlBuilder) {
        // let column: &'a dyn Column;
        // let str: &str;

        let (column, r#str) = match self {
            OrderByItem::ASC(v) => (v, "asc"),
            OrderByItem::DESC(v) => (v, "desc"),
        };

        column.column(sql_builder);
        sql_builder.write_sql(" ");
        sql_builder.write_sql(str);
    }
}
