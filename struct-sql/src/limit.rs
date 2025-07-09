use crate::sql_builder::SqlBuilder;

pub type Limit = u32;

pub trait TLimit {
    fn limit(&self, sql_builder: &mut SqlBuilder);
}

impl TLimit for Limit {
    fn limit(&self, sql_builder: &mut SqlBuilder) {
        sql_builder.write_sql(" limit ");
        sql_builder.write_sql(self.to_string().as_str())
    }
}
