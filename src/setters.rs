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

fn get_expressions(ast_struct: DataStruct) -> (
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    Vec<TokenStream>
) {
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

    let property_types = collected
        .iter()
        .cloned()
        .map(define_property_type)
        .collect::<Vec<TokenStream>>();

    (apperance_setters, layout_setters, apperance_cases, layout_cases, property_types)
}

fn switch_case(field: StructField) -> TokenStream {
    let name = field.name;

    let setter_name = "set_".to_string() + name.to_string().as_ref();
    let setter_name = Ident::new(setter_name.as_str(), Span::call_site());

    quote! {
        stringify!(#name) => setters::#setter_name(self, name.to_string(), property),
    }
}

fn setter_fn_appearance(field: StructField) -> TokenStream {
    let field_type = field.ftype;
    let name = field.name;

    let setter_name = "set_".to_string() + name.to_string().as_ref();
    let setter_name = Ident::new(setter_name.as_str(), Span::call_site());

    quote!{
        pub fn #setter_name(
            properties: &mut Properties,
            key: String,
            value: Appearance
        ) -> Result<(), PropertyError> {
            if let Some(expected) = extract!(Appearance::#field_type(_), value) {
                let wrap_value = Appearance::#field_type(expected);
                set_appearance_without_check(properties, key, wrap_value);
                Ok(())
            } else {
                Err(expected_type_error(key))
            }
        }
    }
}

fn setter_fn_layout(field: StructField) -> TokenStream {
    // Fixes problem with `class_case` method if Inflector
    // Also check enum types for BorderWidth align
    let (_, enum_name) = property_class_case(field.name.to_string().as_str());
    let field_type = field.ftype;
    let name = field.name;

    let setter_name = "set_".to_string() + name.to_string().as_ref();
    let setter_name = Ident::new(setter_name.as_str(), Span::call_site());

    let setter = if field_type.to_string().as_str() != "SharedUnit" {
        quote!{
            if let Some(expected) = extract!(Layout::#field_type(_), value) {
                let wrap_value = FlexStyle::#enum_name(expected);
                set_layout_without_check(properties, key, wrap_value);
                Ok(())
            } else {
                Err(expected_type_error(key))
            }
        }
    } else {
        quote!{
            if let Some(expected) = extract!(Layout::SharedUnit(_), value) {
                set_layout_unit_without_check(properties, key, expected)
            } else {
                Err(expected_type_error(key))
            }
        }
    };

    quote!{
        pub fn #setter_name(
            properties: &mut Properties,
            key: String,
            value: Layout,
        ) -> Result<(), PropertyError> {
            #setter
        }
    }
}

fn match_remove_property(name: &str) -> TokenStream {
    // If value argument is None then remove key from map
    let name = Ident::new(name, Span::call_site());

    quote!{
      if property.is_none() {
        self.#name.0.remove(name).is_some();
        return Ok(());
      }

      let property = property.unwrap();
    }
}

fn define_property_type(field: StructField) -> TokenStream {
    let name = field.name; let ftype = field.ftype;

    quote!{ map.insert(stringify!(#name), stringify!(#ftype)); }
}

pub fn get_impl_trait_tokens(_: Ident, data_struct: DataStruct) -> TokenStream {
    let (
        setters_appearance,
        setters_layout,
        cases_appearance,
        cases_layout, 
        
        property_types
    ) = get_expressions(data_struct);

    let rm_matcher_apperance = match_remove_property("appearance");
    let rm_matcher_layout = match_remove_property("layout");

    let tokens = quote! {
        use std::collections::HashMap;
        use inflector::Inflector;
        use yoga::FlexStyle;
        use traits::TStyle;

        use utils::{
            apperance_keys_contains,
            layout_keys_contains,
            self
        };

        use types::{
            PropertyError,
            PropertyValue,
            Properties,
            Appearance,
            Layout
        };

        lazy_static!{
            static ref PROPERTY_TYPES: HashMap<&'static str, &'static str> = {
                let mut map = HashMap::new();
                #(#property_types)*
                map
            };
        }

        pub fn get_reflect_property_type(key: &str) -> &str {
            PROPERTY_TYPES[key]
        }

        /// Module with vanilla style setters
        pub mod setters {
            use super::{
                PropertyError,
                Properties,
                Appearance,
                FlexStyle,
                Layout,
            };
            
            use super::utils::{
                set_layout_unit_without_check,
                set_appearance_without_check,
                set_layout_without_check,
                expected_type_error,
            };

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
                    })
                }
            }

            fn set_layout_style(&mut self, name: &str, property: Option<Layout>) -> Result<(), PropertyError> {
                #rm_matcher_layout

                match name {
                    #(#cases_layout)*
                    _ => Err(PropertyError::InvalidKey {
                        key: name.to_string()
                    })
                }
            }
        }
    };

    // println!("{}", tokens.to_string());
    tokens
}
