use crate::column::Column;
use crate::condition::Condition;
use crate::sql_builder::SqlBuilder;

pub trait TWhere<'a> {
    fn r#where(self, sql_builder: &mut SqlBuilder<'a>);
}
pub type Where<'a, COLUMN> = Vec<Condition<'a, COLUMN>>;

impl<'a, COLUMN: Column> TWhere<'a> for Where<'a, COLUMN> {
    fn r#where(self, sql_builder: &mut SqlBuilder<'a>) {
        if self.len()==0 { return }

        sql_builder.write_sql(" where");

        self.into_iter().for_each(|condition| {
            condition.condition(sql_builder)
        })
    }
}
