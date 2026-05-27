use crate::column::Column;
use crate::condition::TCondition;
use crate::sql_builder::{SqlArg, SqlBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    In,
    Nin,
    Between,
    NotBetween,
    BetweenSymmetric,
    NotBetweenSymmetric,
    IsTrue,
    IsNotTrue,
    IsFalse,
    IsNotFalse,
    IsUnknown,
    IsNotUnknown,
    IsNull,
    IsNotNull,
    Like,
    ILike,
    ArrayOverlap,
    StDWithin,
    DistanceGte,
}

impl Operator {
    pub fn to_sql(&self) -> &str {
        match self {
            Operator::Eq => "=",
            Operator::Neq => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
            Operator::In => "in",
            Operator::Nin => "not in",
            Operator::Between => "between",
            Operator::NotBetween => "not between",
            Operator::BetweenSymmetric => "between symmetric",
            Operator::NotBetweenSymmetric => "not between symmetric",
            Operator::IsTrue => "is true",
            Operator::IsNotTrue => "is not true",
            Operator::IsFalse => "is false",
            Operator::IsNotFalse => "is not false",
            Operator::IsUnknown => "is unknown",
            Operator::IsNotUnknown => "is not unknown",
            Operator::IsNull => "is null",
            Operator::IsNotNull => "is not null",
            Operator::Like => "like",
            Operator::ILike => "ilike",
            Operator::ArrayOverlap => "&&",
            _ => "",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConditionV2<'a, COLUMN: Column> {
    COA(COLUMN, Operator, Vec<SqlArg<'a>>),
    CO(COLUMN, Operator),
    AND,
    OR,
    Raw(String),
    Conditions(Vec<ConditionV2<'a, COLUMN>>),
}

impl<'a, COLUMN: Column> TCondition<'a> for ConditionV2<'a, COLUMN> {
    fn condition(&self, sql_builder: &mut SqlBuilder<'a>) {
        match self {
            ConditionV2::Conditions(v) => {
                sql_builder.write_sql(" ");
                // conditions(v, sql_builder);
                sql_builder.write_sql("(");
                v.iter()
                    .for_each(|condition| condition.condition(sql_builder));
                sql_builder.write_sql(" )");
            }
            ConditionV2::AND => sql_builder.write_sql(" and"),
            ConditionV2::OR => sql_builder.write_sql(" or"),
            ConditionV2::Raw(v) => {
                sql_builder.write_sql(" ");
                sql_builder.write_sql(v);
            }
            ConditionV2::COA(column, operator, args) => {
                sql_builder.write_sql(" ");
                debug_assert!(
                    column.is_indexed(),
                    "{column:#?} not indexed. try add index for this column.",
                );
                match operator {
                    Operator::Eq
                    | Operator::Neq
                    | Operator::Gt
                    | Operator::Lt
                    | Operator::Gte
                    | Operator::Lte
                    | Operator::Like
                    | Operator::ILike
                    | Operator::ArrayOverlap => {
                        column.column(sql_builder);
                        sql_builder.write_sql(" ");
                        sql_builder.write_sql(operator.to_sql());
                        sql_builder.write_sql(" ");
                        sql_builder.push_arg(args[0]);
                    }
                    Operator::In | Operator::Nin => {
                        column.column(sql_builder);
                        sql_builder.write_sql(" ");
                        sql_builder.write_sql(if operator == &Operator::In { "=" } else { "!=" });
                        sql_builder.write_sql(" ANY(");
                        sql_builder.push_args_iter(args.iter());
                        sql_builder.write_sql(")");
                    }
                    Operator::Between
                    | Operator::NotBetween
                    | Operator::BetweenSymmetric
                    | Operator::NotBetweenSymmetric => {
                        column.column(sql_builder);
                        sql_builder.write_sql(" ");
                        sql_builder.write_sql(operator.to_sql());
                        sql_builder.write_sql(" ");
                        sql_builder.push_arg(args[0]);
                        sql_builder.write_sql(" and ");
                        sql_builder.push_arg(args[1]);
                    }
                    Operator::StDWithin => {
                        sql_builder.write_sql("ST_DWithin(");
                        column.column(sql_builder);
                        sql_builder.write_sql(", ST_MakePoint(");
                        sql_builder.push_arg(args[0]);
                        sql_builder.write_sql(", ");
                        sql_builder.push_arg(args[1]);
                        sql_builder.write_sql(")::geography, ");
                        sql_builder.push_arg(args[2]);
                        sql_builder.write_sql(")");
                    }
                    Operator::DistanceGte => {
                        column.column(sql_builder);
                        sql_builder.write_sql("<-> ST_MakePoint(");
                        sql_builder.push_arg(args[0]);
                        sql_builder.write_sql(", ");
                        sql_builder.push_arg(args[1]);
                        sql_builder.write_sql(") >= (ST_MAKEPOINT(");
                        sql_builder.push_arg(args[0]);
                        sql_builder.write_sql(", ");
                        sql_builder.push_arg(args[1]);
                        sql_builder.write_sql(")::geography <-> ST_MAKEPOINT(");
                        sql_builder.push_arg(args[2]);
                        sql_builder.write_sql(", ");
                        sql_builder.push_arg(args[3]);
                        sql_builder.write_sql(")::geography)");
                    }
                    _ => {
                        column.column(sql_builder);
                        sql_builder.write_sql(" ");
                        sql_builder.write_sql(operator.to_sql());
                    }
                }
            }
            ConditionV2::CO(column, operator) => {
                sql_builder.write_sql(" ");
                debug_assert!(
                    column.is_indexed(),
                    "{column:#?} not indexed. try add index for this column.",
                );
                column.column(sql_builder);
                sql_builder.write_sql(" ");
                sql_builder.write_sql(operator.to_sql());
            }
        }
    }
}

#[macro_export]
macro_rules! condition {
    ($column:expr, $operator:expr) => {
        $crate::condition_v2::ConditionV2::CO($column, $operator)
    };
    ($column:expr, $operator:expr, $($arg:expr),+) => {
        $crate::condition_v2::ConditionV2::COA($column, $operator, vec![$($arg),+])
    };
}
