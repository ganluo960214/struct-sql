use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::{Data, DataStruct, DeriveInput, Error, Expr, ExprLit, Fields, FieldsNamed, Lit, Meta, MetaNameValue, token, LitStr, LitBool};

enum StructSql {
    Table(Table),
    VirtualView(VirtualView),
}

const ATTR_TABLE: &str = "struct_sql_table";
const ATTR_VIEW: &str = "struct_sql_virtual_view";

const ATTR_COLUMN: &str = "struct_sql_column";
const ATTR_COLUMN_META_PATH_SKIP_LITERAL: &str = "skip";
const ATTR_COLUMN_META_PATH_NAME_LITERAL: &str = "name";
const ATTR_COLUMN_META_PATH_IS_INDEXED_LITERAL: &str = "is_indexed";
const ATTR_COLUMN_META_PATH_IS_PRIMARY_KEY_LITERAL: &str = "is_primary_key";
const ATTR_COLUMN_META_PATH_IS_UNIQUE_LITERAL: &str = "is_unique";

#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_META_FILE_PATH: &str = "file_path";
#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_MAIN_TABLE: &str = "struct_sql_virtual_view_main_table";
#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_MAIN_TABLE_META_FILE_PATH: &str = "file_path";

#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_JOIN: &str = "struct_sql_virtual_view_join";
#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_JOIN_META_JOIN_TYPE: &str = "join_type";
#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_JOIN_META_LEFT: &str = "left";
#[allow(dead_code)]
const ATTR_VIRTUAL_VIEW_JOIN_META_RIGHT: &str = "right";

struct Table {
    derive_struct_name: String,
    derive_struct_fields: FieldsNamed,
    sql_table_name: String,
}
impl A for Table {
    fn struct_ident(&self) -> Ident {
        Ident::new(self.derive_struct_name.clone().as_str(), Span::call_site())
    }
    fn enum_field_ident(&self) -> Ident {
        Ident::new(
            format!("{}Field", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
}
impl Table {
    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        let fields_attr: Vec<DeriveStructFieldAttrColumn> =
            DeriveStructFieldAttrColumn::from_fields_named_to_vec(
                self.derive_struct_fields.clone(),
            )?;

        // struct impl StructSqlTable trait
        let struct_impl_struct_sql_table_token = generate_impl_struct_sql_table(self);

        // // struct impl from_row
        let struct_impl_from_row = generate_struct_impl_from_row_and_rows(self, &fields_attr);

        // struct fields to columns
        let struct_fields_to_columns_token = generate_enum_field(self, &fields_attr);

        Ok(quote! {
            #struct_impl_struct_sql_table_token
            #struct_impl_from_row
            #struct_fields_to_columns_token
        })
    }
}

struct VirtualView {
    derive_struct_name: String,
    // derive_struct_fields: FieldsNamed,
}
impl A for VirtualView {
    /// X
    fn struct_ident(&self) -> Ident {
        Ident::new(self.derive_struct_name.as_str(), Span::call_site())
    }
    /// enum XField
    fn enum_field_ident(&self) -> Ident {
        Ident::new(
            format!("{}Field", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
}
impl VirtualView {
    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        // let fields_attr = DeriveStructFieldAttrVirtualView::from_fields_named_to_vec(
        //     self.derive_struct_fields.clone(),
        // )?;

        Ok(quote! {})
    }
}

trait A {
    // X
    fn struct_ident(&self) -> Ident;
    // enum XField
    fn enum_field_ident(&self) -> Ident;
}

#[derive(Debug, Default)]
struct DeriveStructFieldAttrColumn {
    derive_struct_field_name: String,

    // ATTR_COLUMN_META_PATH_NAME_LITERAL
    sql_field_name: Option<String>,
    // ATTR_COLUMN_META_PATH_IS_INDEXED_LITERAL
    is_indexed: bool,
    // ATTR_COLUMN_META_PATH_IS_PRIMARY_KEY_LITERAL
    is_primary_key_key: bool,
    // ATTR_COLUMN_META_PATH_IS_UNIQUE_LITERAL
    is_unique_key: bool,
}
impl DeriveStructFieldAttrColumn {
    fn enum_item_ident(&self) -> Ident {
        Ident::new(
            self.derive_struct_field_name.clone().as_str(),
            Span::call_site(),
        )
    }

    fn from_fields_named_to_vec(fields_named: FieldsNamed) -> Result<Vec<Self>, Error> {
        let mut columns: Vec<DeriveStructFieldAttrColumn> = Vec::new();
        for field in fields_named.named.iter() {
            if field.ident.is_none() {
                continue;
            }

            let ident = field.ident.clone().unwrap();

            let mut column = DeriveStructFieldAttrColumn {
                derive_struct_field_name: match ident.to_string().strip_prefix("r#") {
                    None => ident.to_string(),
                    Some(v) => v.to_string(),
                },
                sql_field_name: None,
                is_indexed: false,
                is_primary_key_key: false,
                is_unique_key: false,
            };

            let mut column_need_skip: bool = false;

            for field_attr in &field.attrs {
                if !field_attr.path().is_ident(ATTR_COLUMN) {
                    continue;
                }

                let metas: Punctuated<Meta, token::Comma> =
                    field_attr.parse_args_with(Punctuated::parse_terminated)?;

                for meta in metas {
                    let meta_ident = meta.clone().path().require_ident()?.to_string();
                    let meta_ident = meta_ident.as_str();

                    match meta {
                        Meta::Path(v) => {
                            match meta_ident {
                                ATTR_COLUMN_META_PATH_SKIP_LITERAL => {
                                    column_need_skip = true;
                                }
                                _ => return Err(Error::new_spanned(
                                    &v,
                                    format!(
                                        "not support attr '{:?}'",
                                        &meta_ident
                                    )
                                ))
                            }
                        }
                        Meta::NameValue(v) => {
                            match meta_ident {
                                ATTR_COLUMN_META_PATH_NAME_LITERAL => {
                                    column.sql_field_name = Some(a_lit_str(v.value)?.value())
                                }
                                ATTR_COLUMN_META_PATH_IS_PRIMARY_KEY_LITERAL => {
                                    let lit_v = a_lit_bool(v.value)?;
                                    (column.is_primary_key_key, column.is_indexed) =
                                        (lit_v.value, lit_v.value)
                                }
                                ATTR_COLUMN_META_PATH_IS_UNIQUE_LITERAL => {
                                    let lit_v = a_lit_bool(v.value)?;
                                    (column.is_unique_key, column.is_indexed) =
                                        (lit_v.value, lit_v.value)
                                }
                                ATTR_COLUMN_META_PATH_IS_INDEXED_LITERAL => {
                                    column.is_indexed = a_lit_bool(v.value)?.value;
                                }
                                _ => return Err(Error::new_spanned(
                                    v.clone(),
                                    format!(
                                        "not support attr '{:?}'",
                                        v.clone()
                                    )
                                )),
                            }
                        }
                        _ => {
                            return Err(Error::new_spanned(
                                &meta,
                                "unsupported attribute format, expected `key = \"value\"` or `key`",
                            ));
                        }
                    }
                }
            }

            if column_need_skip {
                continue;
            }
            columns.push(column)
        }
        Ok(columns)
    }
}
fn a_lit_str(expr: Expr) -> Result<LitStr,Error> {
    match expr {
        Expr::Lit(ExprLit {
                      lit: Lit::Str(lit_v),
                      ..
                  }) => Ok(lit_v),
        _ =>  Err(Error::new_spanned(
            expr,
            format!(
                "error format,it should be struct_sql_column(..,{}=\"value\",..)",
                ATTR_COLUMN_META_PATH_NAME_LITERAL
            ),
        ))
    }
}
fn a_lit_bool(expr: Expr) -> Result<LitBool,Error> {
    match expr {
        Expr::Lit(ExprLit {
                      lit: Lit::Bool(lit_v),
                      ..
                  }) => Ok(lit_v),
        _ =>  Err(Error::new_spanned(
            expr,
            format!(
                "error format,it should be struct_sql_column(..,{}=true/false,..)",
                ATTR_COLUMN_META_PATH_NAME_LITERAL
            ),
        ))
    }
}

// struct DeriveStructFieldAttrVirtualView {
//     derive_struct_field_name: String,
//     derive_struct_field_type: String,
//
//     // ATTR_VIRTUAL_VIEW_MAIN_TABLE
//     is_virtual_view_main_table: bool,
//
//     // ATTR_VIRTUAL_VIEW_JOIN_META_TYPE
//     virtual_view_join_type: Option<String>,
//     // ATTR_VIRTUAL_VIEW_JOIN_META_LEFT
//     virtual_view_join_left: Option<String>,
//     // ATTR_VIRTUAL_VIEW_JOIN_META_RIGHT
//     virtual_view_join_right: Option<String>,
//     // table
//     table: Option<Table>,
// }

// impl DeriveStructFieldAttrVirtualView {
//     fn a(path: String, ident: Ident) -> Result<StructSql, String> {
//         let path = std::env::current_dir().unwrap().join(path.as_str());
//         if !Path::new(&path).exists() {
//             return Err(format!("file not exists: {}", &path.to_str().unwrap()));
//         }
//
//         let content = fs::read_to_string(&path).unwrap();
//         let file_ast = syn::parse_file(&content).unwrap();
//         for item in file_ast.items.into_iter() {
//             if let Item::Struct(v) = item {
//                 if v.ident == ident {
//                     return Ok(asd(&DeriveInput::from(v)).unwrap().unwrap());
//                 }
//             }
//         }
//
//         Err(format!("struct {} not exists", ident.to_string().as_str()))
//     }
//     fn from_fields_named_to_vec(fields_named: FieldsNamed) -> Result<Vec<Self>, Error> {
//         let mut columns: Vec<DeriveStructFieldAttrVirtualView> = Vec::new();
//         for field in fields_named.named.iter() {
//             if field.ident.is_none() {
//                 continue;
//             }
//
//             let ident = field.ident.clone().unwrap();
//             let ident_type: String = match &field.ty {
//                 Type::Path(v) => v.path.get_ident().unwrap().to_string(),
//                 _ => return Err(Error::new_spanned(&field.ty, "type error".to_string())),
//             };
//
//             let mut column = DeriveStructFieldAttrVirtualView {
//                 derive_struct_field_name: match ident.to_string().strip_prefix("r#") {
//                     None => ident.to_string(),
//                     Some(v) => v.to_string(),
//                 },
//
//                 derive_struct_field_type: ident_type,
//                 is_virtual_view_main_table: false,
//                 virtual_view_join_type: None,
//                 virtual_view_join_left: None,
//                 virtual_view_join_right: None,
//                 table: None,
//             };
//
//             let mut is_set_main_table_or_join: bool = false;
//
//             for field_attr in &field.attrs {
//                 if field_attr.path().is_ident(ATTR_VIRTUAL_VIEW_MAIN_TABLE) {
//                     column.is_virtual_view_main_table = true;
//                     is_set_main_table_or_join = true;
//                 }
//                 if field_attr.path().is_ident(ATTR_VIRTUAL_VIEW_JOIN) {
//                     is_set_main_table_or_join = true;
//                 }
//
//                 let name_values: Result<Punctuated<MetaNameValue, token::Comma>, _> =
//                     field_attr.parse_args_with(Punctuated::parse_terminated);
//                 match name_values {
//                     Ok(nvs) => {
//                         for nv in nvs {
//                             if nv.path.is_ident(ATTR_VIRTUAL_VIEW_META_FILE_PATH) {
//                                 // path = "path/to/file.rs"
//                                 match &nv.value {
//                                     Expr::Lit(ExprLit {
//                                         lit: Lit::Str(lit_v),
//                                         ..
//                                     }) => {
//                                         match DeriveStructFieldAttrVirtualView::a(
//                                             lit_v.value(),
//                                             Ident::new(
//                                                 column.derive_struct_field_type.as_str(),
//                                                 Span::call_site(),
//                                             ),
//                                         ) {
//                                             Err(v) => return Err(Error::new_spanned(lit_v, v)),
//                                             Ok(v) => {
//                                                 if let StructSql::Table(v) = v {
//                                                     column.table = Some(v);
//                                                 } else {
//                                                     return Err(Error::new_spanned(
//                                                         lit_v,
//                                                         "something warn. post it on github"
//                                                             .to_string(),
//                                                     ));
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     _ => {
//                                         return Err(Error::new_spanned(
//                                             &field_attr.meta,
//                                             format!(
//                                                 "error format,it should be {}(file_path = \"path/to/file.rs\")",
//                                                 ATTR_VIRTUAL_VIEW_MAIN_TABLE
//                                             ),
//                                         ));
//                                     }
//                                 }
//                             }
//
//                             if nv.path.is_ident(ATTR_VIRTUAL_VIEW_JOIN_META_JOIN_TYPE) {
//                                 // type = inner/left/right/full
//                                 match &nv.value {
//                                     Expr::Lit(ExprLit {
//                                         lit: Lit::Str(lit_v),
//                                         ..
//                                     }) => column.virtual_view_join_type = Some(lit_v.value()),
//                                     _ => {
//                                         return Err(Error::new_spanned(
//                                             &field_attr.meta,
//                                             format!(
//                                                 "error format,it should be {}(join_type = \"inner/left/right/full\")",
//                                                 ATTR_VIRTUAL_VIEW_JOIN
//                                             ),
//                                         ));
//                                     }
//                                 }
//                             }
//
//                             if nv.path.is_ident(ATTR_VIRTUAL_VIEW_JOIN_META_LEFT) {
//                                 // left = table.column
//                                 match &nv.value {
//                                     Expr::Lit(ExprLit {
//                                         lit: Lit::Str(lit_v),
//                                         ..
//                                     }) => column.virtual_view_join_left = Some(lit_v.value()),
//                                     _ => {
//                                         return Err(Error::new_spanned(
//                                             &field_attr.meta,
//                                             format!(
//                                                 "error format,it should be {}(left = \"table::field_name\")",
//                                                 ATTR_VIRTUAL_VIEW_JOIN
//                                             ),
//                                         ));
//                                     }
//                                 }
//                             }
//
//                             if nv.path.is_ident(ATTR_VIRTUAL_VIEW_JOIN_META_RIGHT) {
//                                 // right = table.column
//                                 match &nv.value {
//                                     Expr::Lit(ExprLit {
//                                         lit: Lit::Str(lit_v),
//                                         ..
//                                     }) => column.virtual_view_join_right = Some(lit_v.value()),
//                                     _ => {
//                                         return Err(Error::new_spanned(
//                                             &field_attr.meta,
//                                             format!(
//                                                 "error format,it should be {}(right = \"table::field_name\")",
//                                                 ATTR_VIRTUAL_VIEW_JOIN
//                                             ),
//                                         ));
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     Err(_) => {
//                         return Err(Error::new_spanned(
//                             &field_attr.meta,
//                             format!(
//                                 "error format,it should be {}(file_path = \"path/to/file.rs\",join_type = \"inner/left/right/full\",left = \"table::field_name\",right = \"table::field_name\"))",
//                                 ATTR_VIRTUAL_VIEW_JOIN
//                             ),
//                         ));
//                     }
//                 }
//             }
//
//             if !is_set_main_table_or_join {
//                 return Err(Error::new_spanned(
//                     field,
//                     format!(
//                         "need attr\n{}({} = \"path/to/file.rs\")\nor\n{}(file_path = \"path/to/file.rs\",join_type = \"inner/left/right/full\",left = \"table::column_name\",right = \"table::column_name\"))",
//                         ATTR_VIRTUAL_VIEW_MAIN_TABLE,
//                         ATTR_VIRTUAL_VIEW_MAIN_TABLE_META_FILE_PATH,
//                         ATTR_VIRTUAL_VIEW_JOIN
//                     ),
//                 ));
//             }
//             columns.push(column)
//         }
//         Ok(columns)
//     }
// }

pub fn struct_sql_macro(root_ast: &DeriveInput) -> Result<TokenStream, Error> {
    let struct_sql: Option<StructSql> = asd(root_ast)?;

    match struct_sql {
        None => Err(Error::new_spanned(
            root_ast,
            "use #[derive(StructSql)] should with attribute(struct_sql_table,struct_sql_virtual_view) like this \n#[derive(StructSql)]\n#[struct_sql_table = \"table_name_in_databases\"] or  #[struct_sql_virtual_view = \"virtual_view\"]",
        )),
        Some(v) => match v {
            StructSql::Table(v) => v.to_token_stream(),
            StructSql::VirtualView(v) => v.to_token_stream(),
        },
    }
}

// todo rename
fn asd(root_ast: &DeriveInput) -> Result<Option<StructSql>, Error> {
    // check root ast is a struct or not and get struct's data
    let struct_fields: &FieldsNamed = match &root_ast.data {
        Data::Struct(DataStruct {
                         fields: Fields::Named(fields),
                         ..
                     }) => fields,
        _ => {
            return Err(Error::new_spanned(
                root_ast,
                "#[derive(StructSql)] only work on A struct",
            ));
        }
    };
    let struct_name: String = root_ast.ident.to_string();

    let mut struct_sql: Option<StructSql> = None;

    // check derive macro attribute(struct_sql_table)/attribute(struct_sql_virtual_view) exists and get attribute(struct_sql_table)/attribute(struct_sql_virtual_view) data
    for attr in &root_ast.attrs {
        if !attr.path().is_ident(ATTR_TABLE) && !attr.path().is_ident(ATTR_VIEW) {
            continue;
        }

        if attr.path().is_ident(ATTR_TABLE) {
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                                    value:
                                    Expr::Lit(ExprLit {
                                                  lit: Lit::Str(lit), ..
                                              }),
                                    ..
                                }) => {
                    struct_sql = Some(StructSql::Table(Table {
                        derive_struct_name: struct_name.clone(),
                        derive_struct_fields: struct_fields.clone(),
                        sql_table_name: lit.value(),
                    }));
                }
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        "use attribute(struct_sql_table) like this #[struct_sql_table = \"table_name_in_databases\"] or #[struct_sql_virtual_view = \"virtual_view\"]",
                    ));
                }
            }
        }

        if attr.path().is_ident(ATTR_VIEW) {
            struct_sql = Some(StructSql::VirtualView(VirtualView {
                derive_struct_name: struct_name.clone(),
                // derive_struct_fields: struct_fields.clone(),
            }));
        }
    }

    Ok(struct_sql)
}

fn generate_struct_impl_from_row_and_rows(
    table: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let struct_ident: Ident = table.struct_ident();
    let enum_field_ident: Ident = table.enum_field_ident();

    let match_enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#enum_field_ident::#name => it.#name = row.get(column.name()),)
        })
        .collect();

    quote! {
        impl #struct_ident {
            pub fn from_row(row: &tokio_postgres::Row) -> Self {
                let mut it = #struct_ident::default();

                row.columns().iter().for_each(|column|{
                    match #enum_field_ident::from_str(column.name()) {
                        Some(field)=>match field {
                            // UserField::id => user.id = row.get(column.name()),
                            #(#match_enum_items_idents)*
                            _ => {}
                        },
                        None=>{}
                    }
                });

                it
            }
            pub fn from_rows(rows: &[tokio_postgres::Row]) -> Vec<Self> {
                let result_vec: Vec<Self> = rows
                    .iter()
                    .map(Self::from_row)
                    .collect();
                result_vec
            }
        }
    }
}

fn generate_impl_struct_sql_table(table: &Table) -> TokenStream {
    let enum_field_ident: Ident = table.enum_field_ident();
    let sql_table_name: String = format!("\"{}\"", table.sql_table_name.clone());
    quote! {
        impl struct_sql::struct_sql_table::StructSqlTable for #enum_field_ident {
            type FIELD = #enum_field_ident;
            fn struct_sql_table(
                &self,
                builder: &mut struct_sql::sql_builder::SqlBuilder,
            ) {
                builder.write_sql(#sql_table_name);
            }
        }
    }
}

fn generate_enum_field(
    r#struct: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let enum_field_ident: Ident = r#struct.enum_field_ident();

    let count1 = Ident::new("count1", Span::call_site());
    let count8 = Ident::new("count8", Span::call_site());

    let enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#name,)
        })
        .chain({
            vec![
                quote!(#count1,),
                quote!(#count8,),
            ]
        })
        .collect();

    let match_enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let name_value = match &column.sql_field_name {
                None => name.to_string(),
                Some(v) => v.to_string(),
            };
            quote!(#enum_field_ident::#name => #name_value,)
        })
        .chain({
            vec![
                quote!(#enum_field_ident::#count1 => "count(1)",),
                quote!(#enum_field_ident::#count8 => "count(*)",),
            ]
        })
        .collect();

    let match_enum_items_idents_a: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let name_value = match &column.sql_field_name {
                None => name.to_string(),
                Some(v) => v.to_string(),
            };
            quote!(#name_value => Some(#enum_field_ident::#name),)
        })
        .collect();

    let match_enum_items_idents_b: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let is_indexed = column.is_indexed || column.is_primary_key_key || column.is_unique_key;
            quote!(#enum_field_ident::#name => #is_indexed,)
        })
        .collect();

    quote!(
        #[derive(Debug,PartialEq, Eq, Hash, Clone)]
        pub enum #enum_field_ident {
            #(#enum_items_idents)*
        }

        impl #enum_field_ident {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #(#match_enum_items_idents)*
                }
            }
            pub fn from_str(str:&str) -> Option<Self> {
                match str {
                    #(#match_enum_items_idents_a)*
                    _ => None,
                }
            }
        }

        impl struct_sql::column::Column for #enum_field_ident {
            fn column(&self, builder: &mut struct_sql::sql_builder::SqlBuilder) {
                builder.write_sql(self.as_str())
            }

            fn is_indexed(&self) -> bool {
                match self {
                    #(#match_enum_items_idents_b)*
                    _ => false
                }
            }
        }
    )
}
