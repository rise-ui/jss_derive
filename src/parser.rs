use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;

use common::{
  appearance_keys_contains,
  field_to_name_and_ty,
  optioned_type,
  StructField,
};

fn get_expressions(ast_struct: DataStruct) -> Vec<TokenStream> {
    let fields = if let Fields::Named(x) = ast_struct.fields {
        x.named
    } else {
        panic!("Only structs with named fields are supported.");
    };

    fields
        .into_iter()
        .filter_map(field_to_name_and_ty)
        .filter_map(|(name, type_path)| optioned_type(name, type_path.segments))
        .map(|value| get_match_expression(value))
        .collect::<Vec<TokenStream>>()
}

fn get_expression_params(field: StructField) -> (Ident, Ident, Ident) {
    let field_type = field.ftype;
    let name = field.name;

    let is_appearance = appearance_keys_contains(&name.to_string());

    let action_method: Ident = if is_appearance {
        Ident::new("set_appearance_style", Span::call_site())
    } else {
        Ident::new("set_layout_style", Span::call_site())
    };

    let wrapper: Ident = if is_appearance {
        Ident::new("Appearance", Span::call_site())
    } else {
        Ident::new("Layout", Span::call_site())
    };

    let wrapper_ty: Ident = {
        let source = field_type.to_string().to_class_case();
        let source = match source.as_str() {
            "BorderRadiu" => "BorderRadius",
            "Transform" => "Transforms",
            "Filter" => "Filters",
            _ => source.as_str(),
        };

        Ident::new(source, Span::call_site())
    };

    (wrapper, wrapper_ty, action_method)
}

fn get_match_expression(field: StructField) -> TokenStream {
    let field_ty = field.ftype.clone();
    let name = field.name.clone();

    let (wrapper, wrapper_ty, action_method) = get_expression_params(field);

    let key_getter = if appearance_keys_contains(&name.to_string()) {
        Ident::new("get_appearance_property_key", Span::call_site())
    } else {
        Ident::new("get_layout_property_key", Span::call_site())
    };

    quote!{
      stringify!(#name) => {
        use self::#wrapper::#wrapper_ty as Content;

        let parsed: #field_ty = deserialize(Box::leak(value))
          .map_err(|error| ParseError::DeserializeError {
            target: stringify!(#field_ty).to_string(),
            error: error.into(),
            key: key.clone(),
          })?;

        let state_name = "default".to_string();
        properties.states.get_mut(&state_name)
          .ok_or(ParseError::StateMissing { name: state_name })
          .and_then(|state| {
            state.#action_method(#key_getter(key.as_str()).unwrap(), Content(parsed))
              .map_err(|error| ParseError::PropertyError { error })
          })?;
      },
    }
}

pub fn get_impl_trait_tokens(_: Ident, data_struct: DataStruct) -> TokenStream {
    let matches = get_expressions(data_struct);

    quote! {
      use types::{Style, ParseError, PropertyKeyInfo, ParseStyleMiddleware};
      use utils::{get_appearance_property_key, get_layout_property_key};
      use traits::*;
      use super::*;

      impl TParseStyleMiddleware for ParseStyleMiddleware {
        fn name(&self) -> String {
          "default".to_string()
        }

        fn process_value(
            &mut self,
            info: PropertyKeyInfo,
            value: Box<Deserializer>,
            properties: &mut Style,
        ) -> Result<(), ParseError> {
          let key = info.name.to_snake_case();

          match key.as_str() {
            #(#matches)*
            _ => {
              // @todo: add error
            },
          }

          Ok(())
        }
      }
    }
}
