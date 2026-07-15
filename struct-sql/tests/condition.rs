// mod common;

// #[cfg(test)]
// mod tests {
//     use bytes::BytesMut;
//     use postgres_types::ToSql;

//     use crate::common;
//     use struct_sql::condition::Condition::{
//         Between, BetweenSymmetric, EQ, GT, GTE, IN, IsFalse, IsNotFalse, IsNotNull, IsNotTrue,
//         IsNotUnknown, IsNull, IsTrue, IsUnknown, LT, LTE, NEQ, NotBetween, NotBetweenSymmetric,
//     };
//     use struct_sql::condition::TCondition;
//     use struct_sql::sql_builder::{SqlArgs, SqlBuilder};
//     use struct_sql_derive::StructSql;

//     ///
//     /// ```sql
//     /// CREATE TABLE IF NOT EXISTS condition_table
//     /// (
//     ///     condition_name integer
//     /// )
//     /// ```
//     ///
//     #[derive(StructSql, Debug, Default)]
//     #[struct_sql_table = "condition_table"]
//     struct ColumnTable {
//         #[struct_sql_column(name = "condition_name", indexed)]
//         condition_name: i32,
//     }
//     const TABLE: ColumnTable = ColumnTable { condition_name: 0 };

//     #[test]
//     fn condition_between() {
//         let mut sb = SqlBuilder::default();
//         let c = Between(
//             ColumnTableField::condition_name,
//             &TABLE.condition_name,
//             &TABLE.condition_name,
//         );
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name between $1 and $2",
//             vec![
//                 &TABLE.condition_name as &(dyn ToSql + Sync),
//                 &TABLE.condition_name,
//             ],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::default(), BytesMut::default());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args as SqlArgs,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_not_between() {
//         let mut sb = SqlBuilder::default();
//         let c = NotBetween(
//             ColumnTableField::condition_name,
//             &TABLE.condition_name,
//             &TABLE.condition_name,
//         );
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name not between $1 and $2",
//             vec![
//                 &TABLE.condition_name,
//                 &TABLE.condition_name as &(dyn ToSql + Sync),
//             ],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );
//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_between_symmetric() {
//         let mut sb = SqlBuilder::default();
//         let c = BetweenSymmetric(
//             ColumnTableField::condition_name,
//             &TABLE.condition_name,
//             &TABLE.condition_name,
//         );
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name between symmetric $1 and $2",
//             vec![
//                 &TABLE.condition_name,
//                 &TABLE.condition_name as &(dyn ToSql + Sync),
//             ],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );
//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_not_between_symmetric() {
//         let mut sb = SqlBuilder::default();
//         let c = NotBetweenSymmetric(
//             ColumnTableField::condition_name,
//             &TABLE.condition_name,
//             &TABLE.condition_name,
//         );
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name not between symmetric $1 and $2",
//             vec![
//                 &TABLE.condition_name,
//                 &TABLE.condition_name as &(dyn ToSql + Sync),
//             ],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_is_true() {
//         let mut sb = SqlBuilder::default();
//         let c = IsTrue(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is true";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_not_true() {
//         let mut sb = SqlBuilder::default();
//         let c = IsNotTrue(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is not true";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_false() {
//         let mut sb = SqlBuilder::default();
//         let c = IsFalse(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is false";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_not_false() {
//         let mut sb = SqlBuilder::default();
//         let c = IsNotFalse(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is not false";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_unknown() {
//         let mut sb = SqlBuilder::default();
//         let c = IsUnknown(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is unknown";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_not_unknown() {
//         let mut sb = SqlBuilder::default();
//         let c = IsNotUnknown(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is not unknown";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_null() {
//         let mut sb = SqlBuilder::default();
//         let c = IsNull(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is null";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_is_not_null() {
//         let mut sb = SqlBuilder::default();
//         let c = IsNotNull(ColumnTableField::condition_name);
//         c.condition(&mut sb);

//         let (left_sql, _) = sb.sql_command();

//         let right_sql = " condition_name is not null";
//         assert_eq!(left_sql, right_sql);
//     }

//     #[test]
//     fn condition_eq() {
//         let mut sb = SqlBuilder::default();
//         let c = EQ(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name = $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );
//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_neq() {
//         let mut sb = SqlBuilder::default();
//         let c = NEQ(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name != $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_gt() {
//         let mut sb = SqlBuilder::default();
//         let c = GT(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name > $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_lt() {
//         let mut sb = SqlBuilder::default();
//         let c = LT(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name < $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_gte() {
//         let mut sb = SqlBuilder::default();
//         let c = GTE(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name >= $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::new(), BytesMut::new());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_lte() {
//         let mut sb = SqlBuilder::default();
//         let c = LTE(ColumnTableField::condition_name, &TABLE.condition_name);
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name <= $1",
//             vec![&TABLE.condition_name as &(dyn ToSql + Sync)],
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::default(), BytesMut::default());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     #[test]
//     fn condition_in() {
//         let mut sb = SqlBuilder::default();
//         let sql_args: SqlArgs = vec![&TABLE.condition_name, &TABLE.condition_name];

//         let c = IN(ColumnTableField::condition_name, sql_args.clone());
//         c.condition(&mut sb);

//         let (left_sql, left_args) = sb.sql_command();

//         let (right_sql, right_args) = (
//             " condition_name in ($1, $2)",
//             vec![&0 as &(dyn ToSql + Sync), &0] as SqlArgs,
//         );

//         assert_eq!(left_sql, right_sql);

//         let (mut left_bytes_mut, mut right_bytes_mut) = (BytesMut::default(), BytesMut::default());

//         writes_sql_args_to_bytes(
//             right_sql,
//             left_args,
//             right_args,
//             &mut left_bytes_mut,
//             &mut right_bytes_mut,
//         );

//         assert_eq!(left_bytes_mut, right_bytes_mut);
//     }

//     fn writes_sql_args_to_bytes(
//         sql: &str,
//         left_args: SqlArgs,
//         right_args: SqlArgs,
//         left_bytes_mut: &mut BytesMut,
//         right_bytes_mut: &mut BytesMut,
//     ) {
//         let sql = format!("{}{}", "select * from condition_table where", sql);

//         common::writes_sql_args_to_bytes(
//             sql.as_str(),
//             left_args,
//             right_args,
//             left_bytes_mut,
//             right_bytes_mut,
//         )
//     }
// }
