use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::iter::FromIterator;
use syn::export::TokenStream;
use syn::parse::ParseBuffer;
use syn::punctuated::Punctuated;
use syn::token::{Colon, Eq, Gt, Lt};
use syn::Token;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, GenericParam,
    Generics, Path, PathSegment, Type, TypeParam, TypePath, Visibility,
};

const DB_STATE_GENERIC_IDENT: &str = "__State";
const DB_STATE_FIELD_IDENT: &str = "__state";

const DB_STATE_SAVED_PATH: &str = "crate::db::Saved";
const DB_STATE_UNSAVED_PATH: &str = "crate::db::Unsaved";

const fn db_state_default_path() -> &'static str {
    DB_STATE_UNSAVED_PATH
}

struct Attrs<'a> {
    attrs: &'a [Attribute],
}

impl ToTokens for Attrs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(self.attrs)
    }
}

impl<'a> From<&'a [Attribute]> for Attrs<'a> {
    fn from(attrs: &'a [Attribute]) -> Self {
        Self { attrs }
    }
}

#[proc_macro]
pub fn db(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let data = if let Data::Struct(s) = &mut input.data {
        s
    } else {
        panic!("Expected a struct");
    };

    add_generic(&mut input.generics, state_generic());
    input.generics.lt_token = Some(Lt::default());
    input.generics.gt_token = Some(Gt::default());

    let orig_fields = data.fields.clone();

    add_field(&mut data.fields, state_field());

    let attrs = Attrs::from(input.attrs.as_slice());
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = &data.fields;

    let orig_fields = match orig_fields {
        Fields::Named(named) => named.named,
        _ => panic!("Expected named fields"),
    };

    let orig_fields_params: Punctuated<Field, Token![,]> = orig_fields
        .iter()
        .cloned()
        .map(|mut f| {
            f.vis = Visibility::Inherited;
            f
        })
        .collect();
    let orig_fields_construct: Punctuated<Ident, Token![,]> = orig_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let state_field_ident = state_field_ident();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #attrs
        #vis struct #ident #generics #fields

        impl #impl_generics #ident #ty_generics #where_clause {
            fn __new(#orig_fields_params) -> Self {
                Self {
                    #state_field_ident : crate::db::Unsaved,
                    #orig_fields_construct
                }
            }
        }
    };

    expanded.into()
}

fn state_generic_ident() -> Ident {
    Ident::new(DB_STATE_GENERIC_IDENT, Span::call_site())
}

fn state_generic() -> GenericParam {
    let mut param = TypeParam::from(state_generic_ident());

    param.eq_token = Some(Eq::default());
    param.default = Some(state_default_type());

    GenericParam::Type(param)
}

fn state_default_type() -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: path_punctuated(db_state_default_path()),
        },
    })
}

fn path_punctuated(path: &str) -> Punctuated<PathSegment, Token![::]> {
    path.split("::")
        .into_iter()
        .map(|seg| PathSegment::from(Ident::new(seg, Span::call_site())))
        .collect()
}

fn add_generic(generics: &mut Generics, generic: GenericParam) {
    generics.params.push(generic);
}

fn state_field_ident() -> Ident {
    Ident::new(DB_STATE_FIELD_IDENT, Span::call_site())
}

fn state_field() -> Field {
    Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(state_field_ident()),
        colon_token: Some(Colon::default()),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::from(PathSegment::from(state_generic_ident())),
        }),
    }
}

fn add_field(fields: &mut Fields, mut field: Field) {
    match fields {
        Fields::Named(named) => {
            if field.ident.is_none() {
                panic!("Cannot add unnamed field to named fields");
            }

            named.named.push(field);
        }
        Fields::Unnamed(unnamed) => {
            field.ident = None;
            unnamed.unnamed.push(field);
        }
        Fields::Unit => panic!("Cannot add to unit fields"),
    }
}
