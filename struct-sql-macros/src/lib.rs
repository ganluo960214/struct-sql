mod struct_sql_proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(
    StructSql,
    attributes(
        struct_sql_table,
        struct_sql_virtual_view,
        struct_sql_virtual_view_main_table,
        struct_sql_virtual_view_join,
        struct_sql_column,
        struct_sql_column_features
    )
)]
pub fn struct_sql_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    struct_sql_proc_macro::struct_sql_macro(&ast)
        .unwrap_or_else(|e: syn::Error| e.to_compile_error())
        .into()
}
