use crate::sql_builder::SqlBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum For {
    Update,
    UpdateSkipLocked,
}

impl For {
    pub fn r#for(&self, sql_builder: &mut SqlBuilder) {
        match self {
            For::Update => sql_builder.write_sql(" for update"),
            For::UpdateSkipLocked => sql_builder.write_sql(" for update skip locked"),
        }
    }
}
