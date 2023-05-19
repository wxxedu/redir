use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, token::Comma, Field, Type};

fn map_fields(
    fields: &Punctuated<Field, Comma>,
    f: impl Fn(&Ident, &Type) -> proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_type = field.ty.clone();
            f(&field_name, &field_type)
        })
        .collect::<Vec<_>>()
}

fn write_trait(
    struct_name: &Ident,
    fns: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let trait_name =
        syn::Ident::new(&format!("{}Trait", struct_name), struct_name.span());
    let fns = map_fields(fns, |field_name, field_type| {
        quote::quote! {
            fn #field_name(&self) -> &#field_type;
        }
    });
    quote::quote! {
        pub trait #trait_name {
            #(#fns)*
        }
    }
}

fn write_struct(
    struct_name: &Ident,
    table_name_optional: Option<&Ident>,
    fields: &Punctuated<Field, Comma>,
) -> (proc_macro2::TokenStream, Ident) {
    let trait_name =
        syn::Ident::new(&format!("{}Trait", struct_name), struct_name.span());
    let struct_name = if table_name_optional.is_some() {
        syn::Ident::new(
            &format!("{}Insertable", struct_name),
            struct_name.span(),
        )
    } else {
        struct_name.clone()
    };
    let mut flds = map_fields(fields, |field_name, field_type| {
        quote::quote! {
            #field_name: #field_type
        }
    });
    let mut constructor_fields = map_fields(fields, |field_name, _| {
        quote::quote! {
            #field_name
        }
    });
    let mut methods: Vec<TokenStream> = vec![];
    if table_name_optional.is_none() {
        flds.insert(
            0,
            quote::quote! {
                id: i32
            },
        );
        constructor_fields.insert(
            0,
            quote::quote! {
                id
            },
        );
        methods.push(quote::quote! {
            pub fn id(&self) -> i32 {
                self.id
            }
        });
    }
    methods.push(quote::quote! {
        pub fn new(#(#flds),*) -> Self {
            Self {
                #(#constructor_fields),*
            }
        }
    });
    let fns = map_fields(fields, |field_name, field_type| {
        quote::quote! {
            fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });
    let annotation = match table_name_optional {
        Some(table_name) => {
            quote::quote! {
                #[derive(Debug, Insertable, Queryable, Serialize, Deserialize, Clone)]
                #[diesel(table_name = #table_name)]
            }
        }
        None => quote::quote! {
            #[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
        },
    };
    let mut display: Vec<TokenStream> = Vec::new();
    let mut literal = format!("{} (", struct_name);
    for field in fields {
        let field_name = field.ident.clone().unwrap();
        literal.push_str(&format!("{}: {{}}, ", field_name));
        display.push(quote::quote! {
            self.#field_name
        });
    }
    literal.push_str(")");
    let res = quote::quote! {
        #annotation
        pub struct #struct_name {
            #(#flds),*
        }
        impl #struct_name {
            #(#methods)*
        }
        impl #trait_name for #struct_name {
            #(#fns)*
        }
        impl Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, #literal, #(#display),*)?;
                Ok(())
            }
        }
    };
    (res, struct_name)
}

fn write_from(
    fields: &Punctuated<Field, Comma>,
    queriable: &Ident,
    insertable: &Ident,
) -> TokenStream {
    let flds = map_fields(fields, |field_name, _| {
        quote::quote! {
            #field_name: #queriable.#field_name
        }
    });
    let res = quote::quote! {
        impl From<#queriable> for #insertable {
            fn from(#queriable: #queriable) -> Self {
                Self {
                    #(#flds),*
                }
            }
        }
    };
    res
}

fn write_operator_impl(
    fields: &Punctuated<Field, Comma>,
    database_name: &Ident,
    queriable: &Ident,
    insertable: &Ident,
) -> TokenStream {
    let inserts = map_fields(fields, |field_name, _| {
        quote::quote! {
            #field_name.eq(insertable.#field_name())
        }
    });
    quote::quote! {
        impl Operator<#queriable, #insertable> for Database {
            fn get(&mut self, id_: i32) -> Result<#queriable, DatabaseError> {
                use crate::schema::#database_name::dsl::*;
                let res = #database_name
                    .filter(id.eq(id_))
                    .first::<#queriable>(&mut self.connection)
                    .map_err(|e| DatabaseError::QueryError(e))?;
                Ok(res)
            }

            fn create(&mut self, insertable: impl Into<#insertable>) -> Result<(), DatabaseError> {
                use crate::schema::#database_name::dsl::*;
                diesel::insert_into(#database_name)
                    .values(insertable.into())
                    .execute(&mut self.connection)
                    .map_err(|e| DatabaseError::QueryError(e))?;
                Ok(())
            }

            fn update(&mut self, queriable: #queriable) -> Result<(), DatabaseError> {
                use crate::schema::#database_name::dsl::*;
                let id_ = queriable.id();
                let insertable: #insertable = queriable.into();
                diesel::update(#database_name.filter(id.eq(id_)))
                    .set((#(#inserts),*))
                    .execute(&mut self.connection)
                    .map_err(|e| DatabaseError::QueryError(e))?;
                Ok(())
            }

            fn delete(&mut self, id_: i32) -> Result<(), DatabaseError> {
                use crate::schema::#database_name::dsl::*;
                diesel::delete(#database_name.filter(id.eq(id_)))
                    .execute(&mut self.connection)
                    .map_err(|e| DatabaseError::QueryError(e))?;
                Ok(())
            }
        }
    }
}

#[proc_macro_attribute]
pub fn datamodel(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let table_name = syn::parse_macro_input!(attr as syn::Ident);
    let struct_item = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = struct_item.ident.clone();
    let fields = match struct_item.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("Only named fields are supported"),
    };
    let trait_def = write_trait(&struct_name, &fields);
    let (insertable_def, insertable_name) =
        write_struct(&struct_name, Some(&table_name), &fields);
    let (queryable_def, queryable_name) =
        write_struct(&struct_name, None, &fields);
    let from_insertable =
        write_from(&fields, &queryable_name, &insertable_name);
    let module_name = &format!("{}_module", struct_name).to_lowercase();
    let module_name = syn::Ident::new(module_name, struct_name.span());
    let operator_impl = write_operator_impl(
        &fields,
        &table_name,
        &queryable_name,
        &insertable_name,
    );
    let output = quote::quote! {
        pub use self::#module_name::*;
        pub mod #module_name {
            use super::*;
            use crate::database::{Database, DatabaseError, Operator};
            use diesel::prelude::*;
            use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable};
            use serde::{Deserialize, Serialize};
            use std::fmt::Display;

            #trait_def
            #insertable_def
            #queryable_def
            #from_insertable
            #operator_impl
        }
    };

    output.into()
}
