use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlArgs, SqlBuilder};

#[derive(Debug, Clone)]
pub enum Condition<'a, COLUMN: Column> {
    /// between
    Between(COLUMN, SqlArg<'a>, SqlArg<'a>), // between
    NotBetween(COLUMN, SqlArg<'a>, SqlArg<'a>), // not between
    BetweenSymmetric(COLUMN, SqlArg<'a>, SqlArg<'a>), // between symmetric
    NotBetweenSymmetric(COLUMN, SqlArg<'a>, SqlArg<'a>), // not between symmetric

    /// is
    IsTrue(COLUMN), // is true
    IsNotTrue(COLUMN),    // is not true
    IsFalse(COLUMN),      // in false
    IsNotFalse(COLUMN),   // is not false
    IsUnknown(COLUMN),    // is unknown
    IsNotUnknown(COLUMN), // is not unknown
    IsNull(COLUMN),       // is null
    IsNotNull(COLUMN),    // is not null

    /// link
    AND, // and
    OR, // or

    /// compare
    EQ(COLUMN, SqlArg<'a>), // "="  equal (=) operator
    NEQ(COLUMN, SqlArg<'a>), // "!=" not equal (!= or <>) operator
    GT(COLUMN, SqlArg<'a>),  // ">"  greater than (>) operator
    LT(COLUMN, SqlArg<'a>),  // "<"  less than (<) operator
    GTE(COLUMN, SqlArg<'a>), // ">=" greater than or equal to (>=) operator
    LTE(COLUMN, SqlArg<'a>), // "<=" less than or equal to (<=) operator

    /// in
    IN(COLUMN, SqlArgs<'a>), // "in"
    NIN(COLUMN, SqlArgs<'a>), // "not in"

    ///
    /// like
    ///
    LIKE(COLUMN, SqlArg<'a>),
    ILIKE(COLUMN, SqlArg<'a>),

    ///
    /// array
    ///
    ArrayOverlap(COLUMN, SqlArg<'a>),

    ///
    /// conditions
    ///
    Conditions(Vec<Condition<'a, COLUMN>>),

    ///
    /// raw
    ///
    Raw(String),

    ///
    /// postgis
    ///
    STDWithin(COLUMN, SqlArg<'a>, SqlArg<'a>, SqlArg<'a>),
}

impl<'a, COLUMN: Column> Condition<'a, COLUMN> {
    pub fn condition(&self, sql_builder: &mut SqlBuilder<'a>) {
        sql_builder.write_sql(" ");
        match self {
            Condition::Between(column, begin, end) => {
                condition_between(column, "between", *begin, *end, sql_builder)
            }
            Condition::NotBetween(column, begin, end) => {
                condition_between(column, "not between", *begin, *end, sql_builder)
            }
            Condition::BetweenSymmetric(column, begin, end) => {
                condition_between(column, "between symmetric", *begin, *end, sql_builder)
            }
            Condition::NotBetweenSymmetric(column, begin, end) => {
                condition_between(column, "not between symmetric", *begin, *end, sql_builder)
            }
            Condition::IsTrue(column) => condition_is(column, "is true", sql_builder),

            Condition::IsNotTrue(column) => condition_is(column, "is not true", sql_builder),
            Condition::IsFalse(column) => condition_is(column, "is false", sql_builder),
            Condition::IsNotFalse(column) => condition_is(column, "is not false", sql_builder),
            Condition::IsUnknown(column) => condition_is(column, "is unknown", sql_builder),
            Condition::IsNotUnknown(column) => condition_is(column, "is not unknown", sql_builder),
            Condition::IsNull(column) => condition_is(column, "is null", sql_builder),
            Condition::IsNotNull(column) => condition_is(column, "is not null", sql_builder),
            //
            Condition::AND => sql_builder.write_sql("and"),
            Condition::OR => sql_builder.write_sql("or"),
            //
            Condition::EQ(column, value) => condition_operator(column, "=", *value, sql_builder),
            Condition::NEQ(column, value) => condition_operator(column, "!=", *value, sql_builder),
            Condition::GT(column, value) => condition_operator(column, ">", *value, sql_builder),
            Condition::LT(column, value) => condition_operator(column, "<", *value, sql_builder),
            Condition::GTE(column, value) => condition_operator(column, ">=", *value, sql_builder),
            Condition::LTE(column, value) => condition_operator(column, "<=", *value, sql_builder),

            //
            Condition::IN(column, value) => condition_in(column, "in", value, sql_builder),
            Condition::NIN(column, value) => condition_in(column, "not in", value, sql_builder),

            //
            Condition::LIKE(column, value) => condition_like(column, "like", *value, sql_builder),
            Condition::ILIKE(column, value) => condition_like(column, "ilike", *value, sql_builder),

            //
            Condition::ArrayOverlap(column, value) => {
                condition_array(column, "&&", *value, sql_builder)
            }

            //
            Condition::Conditions(v) => conditions(v, sql_builder),

            Condition::Raw(v) => sql_builder.write_sql(v),
            Condition::STDWithin(v, lon, lat, radius) => {
                st_d_within(v, *lon, *lat, *radius, sql_builder)
            }
        }
    }
}

///
/// condition
///
pub fn conditions<'a, COLUMN: Column>(
    conditions: &Vec<Condition<'a, COLUMN>>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    sql_builder.write_sql(" (");
    conditions
        .iter()
        .for_each(|condition| condition.condition(sql_builder));
    sql_builder.write_sql(" )");
}

///
/// is XXX
///
pub fn condition_is<COLUMN: Column>(column: &COLUMN, str: &str, sql_builder: &mut SqlBuilder) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
}

///
/// between BEGIN and END
///
pub fn condition_between<'a, COLUMN: Column>(
    column: &COLUMN,
    str: &str,
    begin: SqlArg<'a>,
    end: SqlArg<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
    sql_builder.write_sql(" ");
    sql_builder.push_arg(begin);
    sql_builder.write_sql(" and ");
    sql_builder.push_arg(end);
}

///
/// A =  B
///
/// A != B
///
/// A >  B
///
/// A <  B
///
/// A >= B
///
/// A <= B
///
pub fn condition_operator<'a, COLUMN: Column>(
    column: &COLUMN,
    str: &str,
    value: SqlArg<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
    sql_builder.write_sql(" ");
    sql_builder.push_arg(value);
}

///
/// A in [B,C]
///
/// A not in [B,C]
///
pub fn condition_in<'a, COLUMN: Column>(
    column: &COLUMN,
    str: &str,
    value: &SqlArgs<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
    sql_builder.write_sql(" ");
    sql_builder.write_sql("(");
    sql_builder.push_args_iter(value.iter());
    sql_builder.write_sql(")");
}

///
/// a like %b
/// a like b%
/// a like %b%
///
pub fn condition_like<'a, COLUMN: Column>(
    column: &COLUMN,
    str: &str,
    value: SqlArg<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
    sql_builder.write_sql(" ");
    sql_builder.push_arg(value);
}

pub fn condition_array<'a, COLUMN: Column>(
    column: &COLUMN,
    str: &str,
    value: SqlArg<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    debug_assert!(
        column.is_indexed(),
        "{column:#?} not indexed. try add index for this column.",
    );
    column.column(sql_builder);
    sql_builder.write_sql(" ");
    sql_builder.write_sql(str);
    sql_builder.write_sql(" ");
    sql_builder.push_arg(value);
}

pub fn st_d_within<'a, COLUMN: Column>(
    column: &COLUMN,
    lon: SqlArg<'a>,
    lat: SqlArg<'a>,
    radius: SqlArg<'a>,
    sql_builder: &mut SqlBuilder<'a>,
) {
    sql_builder.write_sql("ST_DWithin(");
    column.column(sql_builder);
    sql_builder.write_sql(", ST_MakePoint(");
    sql_builder.push_arg(lon);
    sql_builder.write_sql(", ");
    sql_builder.push_arg(lat);
    sql_builder.write_sql(")::geography, ");
    sql_builder.push_arg(radius);
    sql_builder.write_sql(")");
}
