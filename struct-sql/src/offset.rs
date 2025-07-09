use crate::sql_builder::SqlBuilder;

pub type Offset = u32;

pub trait TOffset {
    fn offset(&self, sql_builder: &mut SqlBuilder);
}

impl TOffset for Offset {
    fn offset(&self, sql_builder: &mut SqlBuilder) {
        sql_builder.write_sql(" offset ");
        sql_builder.write_sql(self.to_string().as_str());
    }
}
