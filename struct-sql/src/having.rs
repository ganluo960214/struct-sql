use crate::condition::{Condition, TCondition};
use crate::sql_builder::SqlBuilder;

pub trait THaving<'a> {
    fn having(&self, sql_builder: &mut SqlBuilder<'a>);
}
pub type Having<'a, COLUMN> = Vec<Condition<'a, COLUMN>>;

impl<'a, C: TCondition<'a>> THaving<'a> for Vec<C> {
    fn having(&self, sql_builder: &mut SqlBuilder<'a>) {
        sql_builder.write_sql(" having");

        self.iter().for_each(|having| having.condition(sql_builder));
    }
}
