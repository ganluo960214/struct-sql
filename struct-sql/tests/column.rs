#[cfg(test)]
mod tests {
    use struct_sql::column::{Column, Columns, TColumns};
    use struct_sql::sql_builder::SqlBuilder;

    use struct_sql_derive::StructSql;

    #[derive(StructSql, Default)]
    #[struct_sql_table = "columns_table"]
    #[allow(dead_code)]
    struct ColumnsTable {
        #[allow(dead_code)]
        #[struct_sql_column(name = "column_name1", is_indexed = true)]
        column_name: i32,
        #[allow(dead_code)]
        column_name2: i32,
        #[allow(dead_code)]
        column_name3: i32,
    }

    #[test]
    fn column() {
        let mut sb: SqlBuilder = SqlBuilder::default();

        ColumnsTableField::column_name.column(&mut sb);

        let right_sql = "column_name1";
        assert_eq!(sb.sql_command().0, right_sql)
    }

    #[test]
    fn columns() {
        let mut sb: SqlBuilder = SqlBuilder::default();

        let columns = [
            ColumnsTableField::column_name,
            ColumnsTableField::column_name2,
            ColumnsTableField::column_name3,
        ];

        columns.columns(&mut sb);

        let right_sql = "column_name1, column_name2, column_name3";
        assert_eq!(sb.sql_command().0, right_sql)
    }
}
