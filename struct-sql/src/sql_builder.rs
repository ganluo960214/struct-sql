use postgres_types::ToSql;
use std::slice::Iter;

#[derive(Debug, Default)]
pub struct SqlBuilder<'a> {
    sql:  String,
    args: SqlArgs<'a>,
}

pub type SqlArg<'a> = &'a (dyn ToSql + Sync);

pub type SqlArgs<'a> = Vec<SqlArg<'a>>;

pub type Sql<'a> = (String, SqlArgs<'a>);

impl<'a> SqlBuilder<'a> {
    pub fn write_sql(&mut self, str: &str) {
        self.sql.push_str(str)
    }

    pub fn push_arg(&mut self, arg: SqlArg<'a>) {
        self.args.push(arg);
        self.sql.push('$');
        self.sql.push_str(self.args.len().to_string().as_str());
    }
    pub fn push_args_iter(&mut self, args: Iter<SqlArg<'a>>) {
        args.enumerate().for_each(|(i, arg)| {
            if i != 0 {
                self.sql.push_str(", ");
            }
            self.push_arg(*arg)
        });
    }

    pub fn sql_command(self) -> Sql<'a> {
        (self.sql, self.args)
    }
}
