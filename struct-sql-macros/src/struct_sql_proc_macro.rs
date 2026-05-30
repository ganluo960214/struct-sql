use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    Data, DataStruct, DeriveInput, Error, Expr, ExprLit, Fields, FieldsNamed, Lit, LitStr, Meta,
    MetaNameValue, Type, token,
};

enum StructSql {
    Table(Table),
    // VirtualView(VirtualView),
    Composite(Composite),
}

const ATTR_TABLE: &str = "struct_sql_table";
const ATTR_COMPOSITE: &str = "struct_sql_composite";
const ATTR_VIEW: &str = "struct_sql_virtual_view";

const ATTR_COLUMN: &str = "struct_sql_column";
const ATTR_COLUMN_META_PATH_SKIP_LITERAL: &str = "skip";
const ATTR_COLUMN_META_PATH_NAME_LITERAL: &str = "name";
const ATTR_COLUMN_META_PATH_SENSITIVE_LITERAL: &str = "sensitive";
const ATTR_COLUMN_META_PATH_INDEXED_LITERAL: &str = "indexed";
const ATTR_COLUMN_META_PATH_PRIMARY_KEY_LITERAL: &str = "primary_key";
const ATTR_COLUMN_META_PATH_UNIQUE_LITERAL: &str = "unique";
const ATTR_COLUMN_META_PATH_COMPOSITE_TYPE_LITERAL: &str = "composite_type";

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

struct Composite {
    derive_struct_name:   String,
    derive_struct_fields: FieldsNamed,
}
impl A for Composite {
    fn struct_ident(&self) -> Ident {
        Ident::new(self.derive_struct_name.clone().as_str(), Span::call_site())
    }
    fn enum_field_ident(&self) -> Ident {
        Ident::new(
            format!("{}Field", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
    fn enum_fields_without_sensitive_ident(&self) -> Ident {
        Ident::new(
            format!("{}FieldsWithoutSensitive", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
    fn enum_all_fields_ident(&self) -> Ident {
        Ident::new(
            format!("{}AllFields", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
}
impl Composite {
    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        let fields_attr: Vec<DeriveStructFieldAttrColumn> =
            DeriveStructFieldAttrColumn::from_fields_named_to_vec(
                self.derive_struct_fields.clone(),
            )?;

        // struct fields to columns
        let struct_fields_to_columns_token = generate_composite_enum_field(self, &fields_attr);

        Ok(quote! {
            #struct_fields_to_columns_token
        })
    }
}

struct Table {
    derive_struct_name:   String,
    derive_struct_fields: FieldsNamed,
    sql_table_name:       String,
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
    fn enum_fields_without_sensitive_ident(&self) -> Ident {
        Ident::new(
            format!("{}FieldsWithoutSensitive", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
    fn enum_all_fields_ident(&self) -> Ident {
        Ident::new(
            format!("{}AllFields", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
}
impl Table {
    fn empty_select_ident(&self) -> Ident {
        Ident::new(
            format!("{}EmptySelect", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
    fn empty_selectv2_ident(&self) -> Ident {
        Ident::new(
            format!("{}EmptySelectV2", self.derive_struct_name).as_str(),
            Span::call_site(),
        )
    }
    // fn empty_update_ident(&self) -> Ident {
    //     Ident::new(
    //         format!("{}EmptyUpdate", self.derive_struct_name).as_str(),
    //         Span::call_site(),
    //     )
    // }
    // fn empty_insert_ident(&self) -> Ident {
    //     Ident::new(
    //         format!("{}EmptyInsert", self.derive_struct_name).as_str(),
    //         Span::call_site(),
    //     )
    // }
    // fn empty_delete_ident(&self) -> Ident {
    //     Ident::new(
    //         format!("{}EmptyDelete", self.derive_struct_name).as_str(),
    //         Span::call_site(),
    //     )
    // }

    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        let fields_attr: Vec<DeriveStructFieldAttrColumn> =
            DeriveStructFieldAttrColumn::from_fields_named_to_vec(
                self.derive_struct_fields.clone(),
            )?;

        // struct impl StructSqlTable trait
        let struct_impl_struct_sql_table_token = generate_impl_struct_sql_table(self, &fields_attr);

        // // struct impl from_row
        let struct_impl_from_row = generate_struct_impl_from_row_and_rows(self, &fields_attr);

        // struct fields to columns
        let struct_fields_to_columns_token = generate_enum_field(self, &fields_attr);

        // empty select,update,delete,insert
        let empty_select_update_delete_insert =
            generate_empty_select_update_delete_insert(self, &fields_attr);

        Ok(quote! {
            #struct_impl_struct_sql_table_token
            #struct_impl_from_row
            #struct_fields_to_columns_token
            #empty_select_update_delete_insert
        })
    }
}

// struct VirtualView {
//     derive_struct_name: String,
//     // derive_struct_fields: FieldsNamed,
// }
// impl A for VirtualView {
//     ///
//     fn struct_ident(&self) -> Ident {
//         Ident::new(self.derive_struct_name.as_str(), Span::call_site())
//     }
//     /// enum Field
//     fn enum_field_ident(&self) -> Ident {
//         Ident::new(
//             format!("{}Field", self.derive_struct_name).as_str(),
//             Span::call_site(),
//         )
//     }
// }
// impl VirtualView {
//     fn to_token_stream(&self) -> Result<TokenStream, Error> {
//         // let fields_attr = DeriveStructFieldAttrVirtualView::from_fields_named_to_vec(
//         //     self.derive_struct_fields.clone(),
//         // )?;

//         Ok(quote! {})
//     }
// }

trait A {
    // X
    fn struct_ident(&self) -> Ident;
    // enum Field
    fn enum_field_ident(&self) -> Ident;
    // enum Fields
    fn enum_fields_without_sensitive_ident(&self) -> Ident;
    // enum Fields
    fn enum_all_fields_ident(&self) -> Ident;
}

#[derive(Debug)]
struct DeriveStructFieldAttrColumn {
    derive_struct_field_name: String,
    data_type:                Type,
    // ATTR_COLUMN_META_PATH_NAME_LITERAL
    sql_field_name:           Option<String>,
    // ATTR_COLUMN_META_PATH_SENSITIVE_LITERAL
    is_sensitive:             bool,
    // ATTR_COLUMN_META_PATH_COMPOSITE_TYPE_LITERAL
    composite_type:           Option<String>,
    // ATTR_COLUMN_META_PATH_IS_INDEXED_LITERAL
    is_indexed:               bool,
    // ATTR_COLUMN_META_PATH_IS_PRIMARY_KEY_LITERAL
    is_primary_key_key:       bool,
    // ATTR_COLUMN_META_PATH_IS_UNIQUE_LITERAL
    is_unique_key:            bool,
}

impl DeriveStructFieldAttrColumn {
    fn enum_item_ident(&self) -> Ident {
        Ident::new(
            self.derive_struct_field_name.clone().as_str(),
            Span::call_site(),
        )
    }

    fn enum_item_composite_ident(&self) -> Option<(Ident, Ident, Ident, Type)> {
        self.composite_type.as_ref().map(|v| {
            (
                self.enum_item_ident(),
                Ident::new(
                    format!("{}_composite", self.derive_struct_field_name.clone()).as_str(),
                    Span::call_site(),
                ),
                Ident::new(
                    format!("set_{}_composite", self.derive_struct_field_name.clone()).as_str(),
                    Span::call_site(),
                ),
                syn::parse_str(v)
                    .unwrap_or_else(|_| panic!("Failed to parse \"{v}\" type path string")),
            )
        })
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
                data_type:                field.ty.clone(),
                sql_field_name:           None,
                is_sensitive:             false,
                composite_type:           None,
                is_indexed:               false,
                is_primary_key_key:       false,
                is_unique_key:            false,
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
                        Meta::Path(v) => match meta_ident {
                            ATTR_COLUMN_META_PATH_SKIP_LITERAL => {
                                column_need_skip = true;
                            }
                            ATTR_COLUMN_META_PATH_PRIMARY_KEY_LITERAL => {
                                (column.is_primary_key_key, column.is_indexed) = (true, true)
                            }
                            ATTR_COLUMN_META_PATH_SENSITIVE_LITERAL => column.is_sensitive = true,
                            ATTR_COLUMN_META_PATH_UNIQUE_LITERAL => {
                                (column.is_unique_key, column.is_indexed) = (true, true)
                            }
                            ATTR_COLUMN_META_PATH_INDEXED_LITERAL => column.is_indexed = true,
                            _ => {
                                return Err(Error::new_spanned(
                                    &v,
                                    format!("not support attr '{:?}'", &meta_ident),
                                ));
                            }
                        },
                        Meta::NameValue(v) => match meta_ident {
                            ATTR_COLUMN_META_PATH_NAME_LITERAL => {
                                column.sql_field_name = Some(a_lit_str(v.value)?.value())
                            }
                            ATTR_COLUMN_META_PATH_COMPOSITE_TYPE_LITERAL => {
                                column.composite_type = Some(a_lit_str(v.value)?.value());
                            }
                            _ => {
                                return Err(Error::new_spanned(
                                    &v,
                                    format!("not support attr '{:?}'", &v),
                                ));
                            }
                        },
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

fn a_lit_str(expr: Expr) -> Result<LitStr, Error> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_v),
            ..
        }) => Ok(lit_v),
        _ => Err(Error::new_spanned(
            expr,
            format!(
                "error format,it should be struct_sql_column(..,{ATTR_COLUMN_META_PATH_NAME_LITERAL}=\"value\",..)",
            ),
        )),
    }
}

pub fn struct_sql_macro(root_ast: &DeriveInput) -> Result<TokenStream, Error> {
    let struct_sql: Option<StructSql> = asd(root_ast)?;

    match struct_sql {
        None => Err(Error::new_spanned(
            root_ast,
            "use #[derive(StructSql)] should with attribute(struct_sql_table,struct_sql_virtual_view,struct_sql_composite) like this \n#[derive(StructSql)]\n#[struct_sql_table = \"table_name_in_databases\"] or  #[struct_sql_virtual_view = \"virtual_view\"] or #[struct_sql_composite = \"composite\"]",
        )),
        Some(v) => match v {
            StructSql::Table(v) => v.to_token_stream(),
            // StructSql::VirtualView(v) => v.to_token_stream(),
            StructSql::Composite(v) => v.to_token_stream(),
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

    // check derive macro attribute(struct_sql_table)/attributem n(struct_sql_virtual_view) exists and get attribute(struct_sql_table)/attribute(struct_sql_virtual_view) data
    for attr in &root_ast.attrs {
        if !attr.path().is_ident(ATTR_TABLE)
            && !attr.path().is_ident(ATTR_VIEW)
            && !attr.path().is_ident(ATTR_COMPOSITE)
        {
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
                        derive_struct_name:   struct_name.clone(),
                        derive_struct_fields: struct_fields.clone(),
                        sql_table_name:       lit.value(),
                    }));
                }
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        format!(
                            "use attribute({ATTR_TABLE}) like this #[{ATTR_TABLE} = \"table_name_in_databases\"]"
                        ),
                    ));
                }
            }
        }

        // if attr.path().is_ident(ATTR_VIEW) {
        //     struct_sql = Some(StructSql::VirtualView(VirtualView {
        //         derive_struct_name: struct_name.clone(),
        //         // derive_struct_fields: struct_fields.clone(),
        //     }));
        // }

        if attr.path().is_ident(ATTR_COMPOSITE) {
            struct_sql = Some(StructSql::Composite(
                Composite {
                    derive_struct_name:   struct_name.clone(),
                    derive_struct_fields: struct_fields.clone(),
                }, //     C {
                   //     derive_struct_name: struct_name.clone(),
                   //     derive_struct_fields: struct_fields.clone(),
                   //     sql_table_name: lit.value(),
                   // }
            ));
            // match &attr.meta {
            //     Meta::NameValue(MetaNameValue {
            //         value:
            //             Expr::Lit(ExprLit {
            //                 lit: Lit::Str(lit), ..
            //             }),
            //         ..
            //     }) => {

            //     }
            //     _ => {
            //         return Err(Error::new_spanned(
            //             attr,
            //             format!(
            //                 "use attribute({ATTR_COMPOSITE}) like this #[{ATTR_COMPOSITE} = \"composite_name_in_databases\"]"
            //             ),
            //         ));
            //     }
            // }
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
            let data_type = &column.data_type;
            quote!(#enum_field_ident::#name => it.#name = row.get::<_, #data_type>(column.name()),)
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

fn generate_impl_struct_sql_table(
    table: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let enum_field_ident: Ident = table.enum_field_ident();
    let sql_table_name: String = format!("\"{}\"", table.sql_table_name.clone());

    let primary_key_enum_item: Vec<TokenStream> = columns
        .iter()
        .filter(|column| column.is_primary_key_key)
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#enum_field_ident::#name)
        })
        .collect();

    let match_enum_items_identsa: Vec<TokenStream> = columns
        .iter()
        .filter(|column| column.is_primary_key_key)
        .map(|column| {
            let name = &column.derive_struct_field_name;
            let r#type = &column.data_type;
            // let name = &column.derive_struct_field_name;
            //  "uuid" => row.get::<_, Uuid>(column.name()).to_string(),
            quote!(#name => row.get::<_, #r#type>(column.name()).to_string(),)
        })
        .collect();

    quote! {
        impl struct_sql::struct_sql_table::StructSqlTable for #enum_field_ident {
            type FIELD = #enum_field_ident;
            fn struct_sql_table(
                &self,
                builder: &mut struct_sql::sql_builder::SqlBuilder,
            ) {
                builder.write_sql(#sql_table_name);
            }
            fn primary_key(&self) -> Vec<#enum_field_ident> {
                vec![#(#primary_key_enum_item),*]
            }

            fn primary_key_values_to_string_from_row(row: &tokio_postgres::Row) -> String {
                // debug_assert!()

                row.columns()
                    .iter()
                    .map(|column| match column.name() {
                        // "uuid" => row.get::<_, Uuid>(column.name()).to_string(),
                        #(#match_enum_items_identsa)*
                        _ => "".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            }
        }
    }
}

fn generate_enum_field<Table: A>(
    r#struct: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let enum_field_ident: Ident = r#struct.enum_field_ident();
    let enum_fields_without_sensitive_ident: Ident = r#struct.enum_fields_without_sensitive_ident();
    let enum_all_fields_ident: Ident = r#struct.enum_all_fields_ident();

    let count1 = Ident::new("count1", Span::call_site());
    let count8 = Ident::new("count8", Span::call_site());

    let enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#name,)
        })
        .chain(vec![quote!(#count1,), quote!(#count8,)])
        .chain(
            columns
                .iter()
                .filter_map(|column| column.enum_item_composite_ident())
                .map(|composite_type| {
                    let ident = composite_type.1;
                    let r#type = composite_type.3;
                    quote!(#ident(#r#type),)
                }),
        )
        .chain(
            columns
                .iter()
                .filter_map(|column| column.enum_item_composite_ident())
                .map(|composite_type| {
                    let ident = composite_type.2;
                    let r#type = composite_type.3;
                    quote!(#ident(#r#type),)
                }),
        )
        .collect();

    let match_enum_items_as_str_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let name_value = match &column.sql_field_name {
                None => name.to_string(),
                Some(v) => v.to_string(),
            };
            quote!(#enum_field_ident::#name => builder.write_sql(#name_value),)
        })
        .chain({
            vec![
                quote!(#enum_field_ident::#count1 => builder.write_sql("count(1)"),),
                quote!(#enum_field_ident::#count8 => builder.write_sql("count(*)"),),
            ]
        })
        .collect();

    let match_enum_items_composite_as_str_idents: Vec<TokenStream> = columns
        .iter()
        .filter_map(|column| column.enum_item_composite_ident())
        .map(|composite_type| {
            let ident1 = composite_type.0;
            let ident2 = composite_type.1;
            quote!(#enum_field_ident::#ident2(v) => {
                builder.write_sql("(");
                #enum_field_ident::#ident1.as_str(builder);
                builder.write_sql(").");
                v.as_str(builder)
            },)
        })
        .collect();

    let match_enum_set_items_composite_as_str_idents: Vec<TokenStream> = columns
        .iter()
        .filter_map(|column| column.enum_item_composite_ident())
        .map(|composite_type| {
            let ident1 = composite_type.0;
            let ident2 = composite_type.2;
            quote!(#enum_field_ident::#ident2(v) => {
                #enum_field_ident::#ident1.as_str(builder);
                builder.write_sql(".");
                v.as_str(builder)
            },)
        })
        .collect();

    let match_enum_items_from_str_idents: Vec<TokenStream> = columns
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

    let match_enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#enum_field_ident::#name,)
        })
        .collect();
    let match_enum_items_idents_count = columns.len();
    let match_enum_not_sensitive_items_idents: Vec<TokenStream> = columns
        .iter()
        .filter(|v| !v.is_sensitive)
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#enum_field_ident::#name,)
        })
        .collect();

    let match_enum_indexed_items_idents: Vec<Vec<TokenStream>> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let is_indexed = column.is_indexed || column.is_primary_key_key || column.is_unique_key;
            let mut vec = vec![quote!(#enum_field_ident::#name => #is_indexed,)];
            if let Some(v) = column.enum_item_composite_ident() {
                let ident1 = v.1;
                vec.push(quote!(#enum_field_ident::#ident1(_) => #is_indexed,));
            }
            vec
        })
        .collect();

    quote!(
        #[derive(Debug,PartialEq, Eq, Hash, Clone)]
        pub enum #enum_field_ident {
            #(#enum_items_idents)*
        }

        pub static #enum_all_fields_ident:[#enum_field_ident;#match_enum_items_idents_count] = [
             #(#match_enum_items_idents)*
        ];

        pub static #enum_fields_without_sensitive_ident:std::sync::LazyLock<Vec<#enum_field_ident>> = std::sync::LazyLock::new(||vec![
             #(#match_enum_not_sensitive_items_idents)*
        ]);

        impl #enum_field_ident {
            pub fn as_str(&self, builder: &mut struct_sql::sql_builder::SqlBuilder) {
                match self {
                    #(#match_enum_items_as_str_idents)*
                    #(#match_enum_items_composite_as_str_idents)*
                    #(#match_enum_set_items_composite_as_str_idents)*
                }
            }
            pub fn from_str(str:&str) -> Option<Self> {
                match str {
                    #(#match_enum_items_from_str_idents)*
                    _ => None,
                }
            }
        }

        impl struct_sql::column::Column for #enum_field_ident {
            fn column(&self, builder: &mut struct_sql::sql_builder::SqlBuilder) {
                self.as_str(builder)
            }

            fn is_indexed(&self) -> bool {
                match self {
                    #(#(#match_enum_indexed_items_idents)*)*
                    _ => false
                }
            }
        }
    )
}

fn generate_composite_enum_field<Table: A>(
    r#struct: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let enum_field_ident: Ident = r#struct.enum_field_ident();

    let enum_items_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            quote!(#name,)
        })
        .collect();

    let match_enum_items_as_str_idents: Vec<TokenStream> = columns
        .iter()
        .map(|column| {
            let name = &column.enum_item_ident();
            let name_value = match &column.sql_field_name {
                None => name.to_string(),
                Some(v) => v.to_string(),
            };
            quote!(#enum_field_ident::#name => builder.write_sql(#name_value),)
        })
        .collect();

    quote!(
        #[derive(Debug,PartialEq, Eq, Hash, Clone)]
        pub enum #enum_field_ident {
            #(#enum_items_idents)*
        }

        impl #enum_field_ident {
            pub fn as_str(&self, builder: &mut struct_sql::sql_builder::SqlBuilder) {
                match self {
                    #(#match_enum_items_as_str_idents)*
                }
            }
        }

        impl struct_sql::column::Column for #enum_field_ident {
            fn column(&self, builder: &mut struct_sql::sql_builder::SqlBuilder) {
                self.as_str(builder)
            }

            fn is_indexed(&self) -> bool {
                false
            }
        }
    )
}

fn generate_empty_select_update_delete_insert(
    r#struct: &Table,
    columns: &[DeriveStructFieldAttrColumn],
) -> TokenStream {
    let empty_select_ident = r#struct.empty_select_ident();
    let empty_selectv2_ident = r#struct.empty_selectv2_ident();
    // let empty_update_ident = r#struct.empty_update_ident();
    // let empty_insert_ident = r#struct.empty_insert_ident();
    // let empty_delete_ident = r#struct.empty_delete_ident();

    let enum_field_ident: Ident = r#struct.enum_field_ident();
    let first_column = columns[0].enum_item_ident();

    quote!(
        pub const #empty_select_ident: struct_sql::sql_select::Select<#enum_field_ident, struct_sql::r#where::Where<#enum_field_ident>> = struct_sql::sql_select::Select {
            columns: vec![],
            from: #enum_field_ident::#first_column,
            r#where: vec![],
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
            offset: None,
            r#for: None,
        };
        pub const #empty_selectv2_ident: struct_sql::sql_select::Select<#enum_field_ident, struct_sql::r#where::WhereV2<#enum_field_ident>> = struct_sql::sql_select::Select {
            columns: vec![],
            from: #enum_field_ident::#first_column,
            r#where: vec![],
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
            offset: None,
            r#for: None,
        };

        // pub const #empty_update_ident: std::sync::LazyLock<struct_sql::sql_update::Update<#enum_field_ident>> =
        //     std::sync::LazyLock::new(|| struct_sql::sql_update::Update {
        //         table:     #enum_field_ident::#first_column,
        //         r#where:   None,
        //         set:       std::collections::HashMap::new(),
        //         returning: None,
        //     });

        // pub const #empty_insert_ident: std::sync::LazyLock<struct_sql::sql_insert::Insert<#enum_field_ident, 0>> =
        //     std::sync::LazyLock::new(|| struct_sql::sql_insert::Insert {
        //         table:        #enum_field_ident::#first_column,
        //         insert_value: struct_sql::sql_insert::InsertValue([], vec![[]]),
        //         on_conflict:  None,
        //         returning:    None,
        //     });

        // pub const #empty_delete_ident: struct_sql::sql_delete::Delete<#enum_field_ident,struct_sql::r#where::Where<#enum_field_ident>> =
        //     struct_sql::sql_delete::Delete {
        //         table:     #enum_field_ident::#first_column,
        //         r#where:   vec![],
        //         returning: None,
        //         _marker:   std::marker::PhantomData,
        //     };
    )
}
