use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;

use common::{
  apperance_keys_contains,
  layout_keys_contains,
  field_to_name_and_ty,
  property_class_case,
  optioned_type,
  StructField,
};

fn get_after_match_setters(name: &str) -> TokenStream {
    let field = Ident::new(name, Span::call_site());

    quote!{
        if properties.#field.0.contains_key(&key) {
            let mut item = properties.#field.0.get_mut(&key).unwrap();
            *item = wrap_value;
        } else {
            properties.#field.0.insert(key, wrap_value);
        }
    }
}


fn switch_case(field: StructField) -> TokenStream {
    let name = field.name;

    quote! {
        stringify!(#name) => setters.set_#name(&mut self, name.to_string(), property),
    }
}

fn setter_fn_appearance(field: StructField) -> TokenStream {
    let expression_setter = get_after_match_setters("appearance");
    let field_type = field.ftype;
    let name = field.name;

    quote!{
        pub fn set_#name(
            properties: &mut Properties,
            key: String,
            value: PropertyValue
        ) -> Result<(), PropertyError> {
            if let Some(expected) = extract!(Appearance::#field_type(_), value) {
                let wrap_value = Appearance::#field_type(expected);
                #expression_setter
                Ok(())
            } else {
                Err(PropertyError::InvalidType {
                    expected: stringify!(#field_type).to_string(),
                    property: stringify!(#name).to_string(),
                })
            }
        },
    }
}

fn setter_fn_layout(field: StructField) -> TokenStream {
    // Fixes problem with `class_case` method if Inflector
    // Also check enum types for BorderWidth align
    let (_, enum_name) = property_class_case(field.name.to_string().as_str());
    let expression_setter = get_after_match_setters("layout");
    let field_type = field.ftype;
    let name = field.name;

    quote!{
        pub fn set_#name(
            properties: &mut Properties,
            key: String,
            value: PropertyValue
        ) -> Result<(), PropertyError> {
            if let Some(expected) = extract!(Layout::#field_type(_), value) {
                let wrap_value = FlexStyle::#enum_name(expected);
                #expression_setter
                Ok(())
            } else {
                Err(PropertyError::InvalidType {
                  expected: stringify!(#field_type).to_string(),
                  property: stringify!(#name).to_string(),
                })
            }
        } 
    }
}

fn match_remove_property(name: &str) -> TokenStream {
    let name = Ident::new(name, Span::call_site());

    quote!{
      // If value argument is None then remove key from map
      if property.is_none() {
        // @TODO: debug log inside this
        if let Some(removed) = self.#name.0.remove(name) {}
        return Ok(());
      }

      // Unwraped value
      let property = property.unwrap();
    }
}

fn get_expressions(ast_struct: DataStruct) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let fields = if let Fields::Named(x) = ast_struct.fields {
        x.named
    } else {
        panic!("Only structs with named fields are supported.");
    };

    let collected = fields
        .into_iter()
        .filter_map(field_to_name_and_ty)
        .filter_map(|(name, type_path)| optioned_type(name, type_path.segments))
        .collect::<Vec<StructField>>();

    let apperance_setters = collected
        .iter()
        .cloned()
        .filter(|x| apperance_keys_contains(&x.name.to_string().as_str()))
        .map(setter_fn_appearance)
        .collect::<Vec<TokenStream>>();

    let layout_setters = collected
        .iter()
        .cloned()
        .filter(|x| layout_keys_contains(&x.name.to_string().as_str()))
        .map(setter_fn_layout)
        .collect::<Vec<TokenStream>>();

    let apperance_cases = collected
        .iter()
        .cloned()
        .filter(|x| apperance_keys_contains(&x.name.to_string().as_str()))
        .map(switch_case)
        .collect::<Vec<TokenStream>>();

    let layout_cases = collected
        .iter()
        .cloned()
        .filter(|x| layout_keys_contains(&x.name.to_string().as_str()))
        .map(switch_case)
        .collect::<Vec<TokenStream>>();

    (apperance_setters, layout_setters, apperance_cases, layout_cases)
}

pub fn get_impl_trait_tokens(struct_id: Ident, data_struct: DataStruct) -> TokenStream {
    let (setters_appearance, setters_layout, cases_appearance, cases_layout) = get_expressions(data_struct);

    let rm_matcher_apperance = match_remove_property("appearance");
    let rm_matcher_layout = match_remove_property("layout");

    quote! {
      use types::{Properties, PropertyError, PropertyValue, Appearance, Layout};
      use utils::{apperance_keys_contains, layout_keys_contains};
      use std::collections::HashMap;
      use inflector::Inflector;
      use yoga::FlexStyle;
      use traits::TStyle;

      pub mod setters {
          #(#setters_appearance)*
          #(#setters_layout)*
      }

      impl TStyle for Properties {
        fn get_apperance_style(&self, name: &str) -> Option<&Appearance> {
          self.appearance.0.get(name)
        }

        fn get_layout_style(&self, name: &str) -> Option<&FlexStyle> {
          self.layout.0.get(name)
        }

        fn set_style(&mut self, name: &str, property: PropertyValue) -> Result<(), PropertyError> {
          if apperance_keys_contains(&name) {
            self.set_apperance_style(name, extract!(PropertyValue::Appearance(_), property))
          } else if layout_keys_contains(&name) {
            self.set_layout_style(name, extract!(PropertyValue::Layout(_), property))
          } else {
            Err(PropertyError::InvalidKey { key: name.to_string() })
          }
        }

        fn set_apperance_style(&mut self, name: &str, property: Option<Appearance>) -> Result<(), PropertyError> {
          #rm_matcher_apperance

          match name {
            #(#cases_appearance)*
            _ => Err(PropertyError::InvalidKey {
              key: name.to_string()
            }),
          }
        }

        fn set_layout_style(&mut self, name: &str, property: Option<Layout>) -> Result<(), PropertyError> {
          #rm_matcher_layout

          match name {
            #(#cases_layout)*
            _ => Err(PropertyError::InvalidKey {
              key: name.to_string()
            }),
          }
        }
      }
    }
}
