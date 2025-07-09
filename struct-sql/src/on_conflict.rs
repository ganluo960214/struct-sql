use crate::column::{Column, ColumnVec, TColumns};
use crate::sql_builder::{SqlArg, SqlBuilder};
use std::collections::HashMap;

pub struct OnConflict<'a, COLUMN: Column> {
    pub on_conflict_target: Option<OnConflictTarget<'a, COLUMN>>,
    pub on_conflict_action: OnConflictAction<'a, COLUMN>,
}

impl<'a, COLUMN: Column> OnConflict<'a, COLUMN> {
    pub fn on_conflict(&self, builder: &mut SqlBuilder<'a>) {
        builder.write_sql(" on conflict");
        if let Some(on_conflict_target) = &self.on_conflict_target {
            on_conflict_target.on_conflict_target(builder);
        }
        self.on_conflict_action.on_conflict_action(builder);
    }
}

pub enum OnConflictTarget<'a, COLUMN: Column> {
    PrimaryKeyOrUniqueColumns(ColumnVec<COLUMN>),
    PrimaryKeyOrUniqueConstraintName(&'a str),
}

impl<COLUMN: Column> OnConflictTarget<'_, COLUMN> {
    pub fn on_conflict_target(&self, builder: &mut SqlBuilder) {
        match self {
            OnConflictTarget::PrimaryKeyOrUniqueColumns(columns) => {
                builder.write_sql(" (");
                columns.columns(builder);
                builder.write_sql(")");
            }
            OnConflictTarget::PrimaryKeyOrUniqueConstraintName(constraint_name) => {
                builder.write_sql(" on constraint ");
                builder.write_sql(constraint_name);
            }
        }
    }
}

// pub type UpdateValue<'a> = HashMap<Column<'a>,SqlArg<'a>>;
pub enum OnConflictAction<'a, COLUMN: Column> {
    DoNothing,
    DoUpdate(DoUpdateValue<'a, COLUMN>),
}

impl<'a, COLUMN: Column> OnConflictAction<'a, COLUMN> {
    pub fn on_conflict_action(&self, builder: &mut SqlBuilder<'a>) {
        match self {
            OnConflictAction::DoNothing => builder.write_sql(" do nothing"),
            OnConflictAction::DoUpdate(v) => v.on_conflict_action(builder),
        }
    }
}

pub type DoUpdateValue<'a, COLUMN> = HashMap<COLUMN, DoUpdateValueKind<'a, COLUMN>>;
pub enum DoUpdateValueKind<'a, COLUMN: Column> {
    Excluded(COLUMN),
    Arg(SqlArg<'a>),
}
impl<'a, COLUMN: Column> DoUpdateValueKind<'a, COLUMN> {
    pub fn do_update_values(&self, sql_builder: &mut SqlBuilder<'a>) {
        match self {
            DoUpdateValueKind::Excluded(column) => {
                sql_builder.write_sql("excluded.");
                column.column(sql_builder);
            }
            DoUpdateValueKind::Arg(arg) => sql_builder.push_arg(*arg),
        }
    }
}

pub trait TOnConflictAction<'a> {
    fn on_conflict_action(&self, sql_builder: &mut SqlBuilder<'a>);
}

impl<'a, COLUMN: Column> TOnConflictAction<'a> for DoUpdateValue<'a, COLUMN> {
    fn on_conflict_action(&self, sql_builder: &mut SqlBuilder<'a>) {
        sql_builder.write_sql(" do update set ");

        self.iter().enumerate().for_each(|(index, (column, value))| {
            if index != 0 {
                sql_builder.write_sql(", ");
            }
            column.column(sql_builder);
            sql_builder.write_sql(" = ");
            value.do_update_values(sql_builder);
        })
    }
}
