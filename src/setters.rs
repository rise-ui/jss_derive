use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;

use common::{
  appearance_keys_contains,
  layout_keys_contains,
  field_to_name_and_ty,
  property_class_case,
  optioned_type,
  StructField,
};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_expressions(ast_struct: DataStruct) -> (
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

    let appearance_setters = collected
        .iter()
        .cloned()
        .filter(|x| appearance_keys_contains(&x.name.to_string().as_str()))
        .map(setter_fn_appearance)
        .collect::<Vec<TokenStream>>();

    let layout_setters = collected
        .iter()
        .cloned()
        .filter(|x| layout_keys_contains(&x.name.to_string().as_str()))
        .map(setter_fn_layout)
        .collect::<Vec<TokenStream>>();

    let appearance_cases = collected
        .iter()
        .cloned()
        .filter(|x| appearance_keys_contains(&x.name.to_string().as_str()))
        .map(|d| switch_case(d, "AppearanceKey"))
        .collect::<Vec<TokenStream>>();

    let layout_cases = collected
        .iter()
        .cloned()
        .filter(|x| layout_keys_contains(&x.name.to_string().as_str()))
        .map(|d| switch_case(d, "LayoutKey"))
        .collect::<Vec<TokenStream>>();

    (appearance_setters, layout_setters, appearance_cases, layout_cases)
}

fn switch_case(field: StructField, key_type: &str) -> TokenStream {
    let name = field.name;

    let setter_name = "set_".to_string() + name.to_string().as_ref();
    let setter_name = Ident::new(setter_name.as_str(), Span::call_site());

    let (etype, ename) = {
        let etype = Ident::new(key_type, Span::call_site());
        let ename = name.to_string().to_camel_case();
        let ename = Ident::new(capitalize(ename.as_str()).as_str(), Span::call_site());

        (etype, ename)
    };

    quote! {
        #etype::#ename => setters::#setter_name(self, name, property),
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
            key: AppearanceKey,
            value: Appearance
        ) -> Result<(), PropertyError> {
            if let Some(expected) = extract!(Appearance::#field_type(_), value) {
                let wrap_value = Appearance::#field_type(expected);
                set_appearance_without_check(properties, key, wrap_value);
                Ok(())
            } else {
                Err(expected_type_error(PropertyKey::Appearance(key)))
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
                Err(expected_type_error(PropertyKey::Layout(key)))
            }
        }
    } else {
        quote!{
            if let Some(expected) = extract!(Layout::SharedUnit(_), value) {
                set_layout_unit_without_check(properties, key, expected)
            } else {
                Err(expected_type_error(PropertyKey::Layout(key)))
            }
        }
    };

    quote!{
        pub fn #setter_name(
            properties: &mut Properties,
            key: LayoutKey,
            value: Layout,
        ) -> Result<(), PropertyError> {
            #setter
        }
    }
}

pub fn get_impl_trait_tokens(_: Ident, data_struct: DataStruct) -> TokenStream {
    let (
        setters_appearance,
        setters_layout,
        cases_appearance,
        cases_layout
    ) = get_expressions(data_struct);

    let tokens = quote! {
        use inflector::Inflector;
        use hashbrown::HashMap;
        use yoga::FlexStyle;
        use traits::TStyle;

        use utils::setter::expected_type_error;
        
        use utils::{
            appearance_keys_contains,
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

        /// Module with vanilla style setters
        pub mod setters {
            use super::{
                PropertyError,
                AppearanceKey,
                PropertyKey,
                Properties,
                Appearance,
                FlexStyle,
                LayoutKey,
                Layout,
            };
            
            use super::utils::setter::{
                set_layout_unit_without_check,
                set_appearance_without_check,
                set_layout_without_check,
                expected_type_error,
            };

            #(#setters_appearance)*
            #(#setters_layout)*
        }

        impl TStyle for Properties {
            fn set_style<T: Into<PropertyValue>>(&mut self, key: PropertyKey, property: T) -> Result<(), PropertyError> {
                let property = property.into();

                match key {
                    PropertyKey::Appearance(key) => {
                        extract!(PropertyValue::Appearance(_), property)
                            .ok_or(expected_type_error(PropertyKey::Appearance(key)))
                            .and_then(|value| self.set_appearance_style(key, value))
                    },

                    PropertyKey::Layout(key) => {
                        extract!(PropertyValue::Layout(_), property)
                            .ok_or(expected_type_error(PropertyKey::Layout(key)))
                            .and_then(|value| self.set_layout_style(key, value))
                    }
                }
            }

            fn set_appearance_style<T: Into<Appearance>>(&mut self, name: AppearanceKey, property: T) -> Result<(), PropertyError> {
                let property = property.into();

                match name {
                    #(#cases_appearance)*
                }
            }

            fn set_layout_style<T: Into<Layout>>(&mut self, name: LayoutKey, property: T) -> Result<(), PropertyError> {
                let property = property.into();

                match name {
                    #(#cases_layout)*
                }
            }

            fn remove_style(&mut self, key: PropertyKey) {
                match key {
                    PropertyKey::Appearance(key) => {
                        self.appearance.0.remove(&key).is_some();
                    },

                    PropertyKey::Layout(key) => {
                        self.expressions.0.remove(&key).is_some();
                        self.layout.0.remove(&key).is_some();
                    },
                }
            }
        }
    };

    // println!("{}", tokens.to_string());
    tokens
}
