use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlBuilder};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub trait TUpdateSet<'a> {
    fn update_set(&self, sql_builder: &mut SqlBuilder<'a>);
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateValue<'a> {
    Value(SqlArg<'a>),
    Increment(SqlArg<'a>),
    Decrement(SqlArg<'a>),
}

impl<'a> From<SqlArg<'a>> for UpdateValue<'a> {
    fn from(arg: SqlArg<'a>) -> Self {
        UpdateValue::Value(arg)
    }
}

impl<'a, T: postgres_types::ToSql + Sync + 'a> From<&'a T> for UpdateValue<'a> {
    fn from(arg: &'a T) -> Self {
        UpdateValue::Value(arg as SqlArg<'a>)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSet<'a, COLUMN>(pub HashMap<COLUMN, UpdateValue<'a>>);

impl<'a, COLUMN: std::cmp::Eq + std::hash::Hash> UpdateSet<'a, COLUMN> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<V: Into<UpdateValue<'a>>>(&mut self, k: COLUMN, v: V) -> Option<UpdateValue<'a>> {
        self.0.insert(k, v.into())
    }
}

impl<'a, COLUMN> IntoIterator for UpdateSet<'a, COLUMN> {
    type Item = (COLUMN, UpdateValue<'a>);
    type IntoIter = std::collections::hash_map::IntoIter<COLUMN, UpdateValue<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'b, COLUMN> IntoIterator for &'b UpdateSet<'a, COLUMN> {
    type Item = (&'b COLUMN, &'b UpdateValue<'a>);
    type IntoIter = std::collections::hash_map::Iter<'b, COLUMN, UpdateValue<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, COLUMN> Deref for UpdateSet<'a, COLUMN> {
    type Target = HashMap<COLUMN, UpdateValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, COLUMN> DerefMut for UpdateSet<'a, COLUMN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, COLUMN: Column + std::cmp::Eq + std::hash::Hash, V: Into<UpdateValue<'a>>, const N: usize> From<[(COLUMN, V); N]> for UpdateSet<'a, COLUMN> {
    fn from(arr: [(COLUMN, V); N]) -> Self {
        let mut map = HashMap::with_capacity(N);
        for (k, v) in arr {
            map.insert(k, v.into());
        }
        UpdateSet(map)
    }
}

impl<'a, COLUMN: Column + std::cmp::Eq + std::hash::Hash, V: Into<UpdateValue<'a>>> FromIterator<(COLUMN, V)> for UpdateSet<'a, COLUMN> {
    fn from_iter<T: IntoIterator<Item = (COLUMN, V)>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for (k, v) in iter {
            map.insert(k, v.into());
        }
        UpdateSet(map)
    }
}

impl<'a, COLUMN: Column> TUpdateSet<'a> for UpdateSet<'a, COLUMN> {
    fn update_set(&self, builder: &mut SqlBuilder<'a>) {
        if self.is_empty() {
            return;
        }
        builder.write_sql(" set ");
        self.iter().enumerate().for_each(|(i, (column, value))| {
            if i != 0 {
                builder.write_sql(", ");
            }
            column.column(builder);
            match value {
                UpdateValue::Value(arg) => {
                    builder.write_sql(" = ");
                    builder.push_arg(*arg);
                }
                UpdateValue::Increment(arg) => {
                    builder.write_sql(" += ");
                    builder.push_arg(*arg);
                }
                UpdateValue::Decrement(arg) => {
                    builder.write_sql(" -= ");
                    builder.push_arg(*arg);
                }
            }
        })
    }
}
