use crate::sql_builder::SqlBuilder;

#[derive(Clone)]
pub enum For {
    Update,
}

impl For {
    pub fn r#for(&self, sql_builder: &mut SqlBuilder) {
        match self {
            For::Update => sql_builder.write_sql(" for update"),
        }
    }
}
