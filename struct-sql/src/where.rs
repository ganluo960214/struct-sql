use crate::condition::{Condition, TCondition};
use crate::condition_v2::ConditionV2;
use crate::sql_builder::SqlBuilder;

pub trait TWhere<'a> {
    fn r#where(&self, sql_builder: &mut SqlBuilder<'a>);
}
pub type Where<'a, COLUMN> = Vec<Condition<'a, COLUMN>>;
pub type WhereV2<'a, COLUMN> = Vec<ConditionV2<'a, COLUMN>>;

impl<'a, C: TCondition<'a>> TWhere<'a> for Vec<C> {
    fn r#where(&self, sql_builder: &mut SqlBuilder<'a>) {
        if self.is_empty() {
            return;
        }

        sql_builder.write_sql(" where");

        self.iter()
            .for_each(|condition| condition.condition(sql_builder))
    }
}

impl<'a, T: TWhere<'a>> TWhere<'a> for Option<T> {
    fn r#where(&self, sql_builder: &mut SqlBuilder<'a>) {
        if let Some(inner) = self {
            inner.r#where(sql_builder);
        }
    }
}
