use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::export::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::{Colon, Eq, Gt, Lt};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, GenericParam, Generics, Path,
    PathSegment, Type, TypeParam, TypePath, Visibility,
};
use syn::{Token, WhereClause};

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

    if !matches!(&data.fields, Fields::Named(_)) {
        panic!("Expected named fields");
    }

    let mut modified_generics = input.generics.clone();
    let mut modified_fields = data.fields.clone();

    prepare_generics(&mut modified_generics);
    prepare_fields(&mut modified_fields);

    let attrs = Attrs::from(input.attrs.as_slice());
    let vis = &input.vis;
    let ident = &input.ident;

    let mut expanded = quote! {
        #attrs
        #vis struct #ident #modified_generics
            #modified_fields
    };

    expanded.append_all(constructor(&data.fields, &input.generics, &ident));

    expanded.into()
}

fn constructor(orig_fields: &Fields, orig_generics: &Generics, ident: &Ident) -> TokenStream2 {
    let (unsaved_impl_generics, unsaved_type_generics, where_clause) = impl_unsaved(&orig_generics);
    let (unsaved_impl_generics, _, _) = unsaved_impl_generics.split_for_impl();
    let (_, unsaved_type_generics, _) = unsaved_type_generics.split_for_impl();

    let (orig_fields_params, orig_fields_constructor) = orig_fields_constructor(&orig_fields);

    let state_field_ident = state_field_ident();

    quote! {
        impl #unsaved_impl_generics #ident #unsaved_type_generics #where_clause {
            fn __new(#orig_fields_params) -> Self {
                Self {
                    #state_field_ident : crate::db::Unsaved,
                    #orig_fields_constructor
                }
            }
        }
    }
}

fn prepare_generics(generics: &mut Generics) {
    add_generic(generics, state_generic());
    generics.lt_token = Some(Lt::default());
    generics.gt_token = Some(Gt::default());
}

fn prepare_fields(fields: &mut Fields) {
    add_field(fields, state_field());
}

// Returns ({Generics for ImplGenerics}, {Generics for TypeGenerics}, {Where clause})
fn impl_unsaved(orig_generics: &Generics) -> (Generics, Generics, Option<WhereClause>) {
    let mut type_generics = orig_generics.clone();
    type_generics.params.push(db_state_unsaved_generic());

    (
        orig_generics.clone(),
        type_generics,
        orig_generics.where_clause.clone(),
    )
}

fn orig_fields_constructor(
    orig_fields: &Fields,
) -> (Punctuated<Field, Token![,]>, Punctuated<Ident, Token![,]>) {
    let params = orig_fields
        .iter()
        .cloned()
        .map(|mut f| {
            f.vis = Visibility::Inherited;
            f
        })
        .collect();
    let constructor = orig_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    (params, constructor)
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

fn db_state_unsaved_ident() -> Ident {
    Ident::new(DB_STATE_UNSAVED_PATH, Span::call_site())
}

fn db_state_unsaved_generic() -> GenericParam {
    GenericParam::Type(TypeParam::from(db_state_unsaved_ident()))
}
