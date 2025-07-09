#[cfg(test)]
mod tests {
    use struct_sql::sql_builder::SqlBuilder;
    use struct_sql::struct_sql_table::StructSqlTable;
    use struct_sql_derive::StructSql;

    #[derive(StructSql, Debug, Default)]
    #[struct_sql_table = "struct_sql_table_table"]
    #[allow(dead_code)]
    struct StructSqlTableTable {
        id: i32,
    }
    const STRUCT_SQL_TABLE: StructSqlTableTableStructSql = StructSqlTableTableStructSql {};
    #[test]
    fn struct_sql_table() {
        let mut sb = SqlBuilder::default();
        STRUCT_SQL_TABLE.struct_sql_table(&mut sb);

        let (left_sql, _) = sb.sql_command();
        let (right_sql, _) = ("\"struct_sql_table_table\"", ());
        assert_eq!(left_sql, right_sql)
    }

}
