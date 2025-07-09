use crate::column::Column;
use crate::condition::Condition;
use crate::sql_builder::SqlBuilder;

pub trait THaving<'a> {
    fn having(self, sql_builder: &mut SqlBuilder<'a>);
}
pub type Having<'a, COLUMN> = Vec<Condition<'a, COLUMN>>;

impl<'a, COLUMN: Column> THaving<'a> for Having<'a, COLUMN> {
    fn having(self, sql_builder: &mut SqlBuilder<'a>) {
        sql_builder.write_sql(" having");

        self.into_iter().for_each(|having| having.condition(sql_builder));
    }
}
