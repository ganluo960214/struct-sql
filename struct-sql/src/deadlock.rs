use crate::sql_builder::{Sql, SqlBuilder};
use std::collections::{BTreeMap, BTreeSet};
use tokio_postgres::Transaction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryLock {
    pub table_name: String,
    pub id:         String,
    pub key_str:    String,
}

impl AdvisoryLock {
    pub fn sql_command(&self) -> Sql<'_> {
        let mut b = SqlBuilder::default();
        b.write_sql("select pg_advisory_xact_lock(hashtextextended(");
        b.push_arg(&self.key_str);
        b.write_sql(", 0))");
        b.sql_command()
    }
}

#[derive(Clone, Debug)]
pub struct LockPlan<'a> {
    id_queries: Vec<(String, Sql<'a>, fn(&tokio_postgres::Row) -> String)>,
}

impl<'a> LockPlan<'a> {
    pub fn from_id_queries(
        id_queries: Vec<(String, Sql<'a>, fn(&tokio_postgres::Row) -> String)>,
    ) -> Self {
        Self { id_queries }
    }

    /// Execute the ID queries within a transaction to fetch the lock IDs,
    /// and then fetch advisory locks to prevent deadlocks.
    pub async fn execute_locks(&self, tx: &Transaction<'a>) -> Result<(), tokio_postgres::Error> {
        let mut locks: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for (table_name, sql, row_to_id) in &self.id_queries {
            let (query_string, args) = sql;

            let rows = tx.query(query_string, &args).await?;

            for row in rows {
                let id: String = row_to_id(&row);
                locks.entry(table_name.clone()).or_default().insert(id);
            }
        }

        let advisory_locks = locks.into_iter().flat_map(|(table_name, ids)| {
            ids.into_iter().map(move |id| AdvisoryLock {
                table_name: table_name.clone(),
                id:         id.clone(),
                key_str:    format!("{}:{}", table_name, id),
            })
        });

        for lock in advisory_locks {
            let (lock_sql, lock_args) = lock.sql_command();
            log::debug!("{:#?}\n{:#?}", lock_sql, lock_args);
            tx.execute(lock_sql.as_str(), &lock_args).await?;
        }

        Ok(())
    }
}

pub trait TLockPlanSql<'a> {
    fn to_lock_query(&self) -> (String, Sql<'a>, fn(&tokio_postgres::Row) -> String);
}

pub fn lock_queries<'a>(
    sqls: Vec<&dyn TLockPlanSql<'a>>,
) -> Vec<(String, Sql<'a>, fn(&tokio_postgres::Row) -> String)> {
    sqls.into_iter().map(|sql| sql.to_lock_query()).collect()
}

pub fn revent_deadlock<'a>(sqls: Vec<&dyn TLockPlanSql<'a>>) -> LockPlan<'a> {
    LockPlan::from_id_queries(lock_queries(sqls))
}
