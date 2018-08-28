use common::{StructField, optioned_type, field_to_name_and_ty };
use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;
use regex::Regex;

lazy_static! {
  static ref APPERANCE_KEYS: Vec<&'static str> = vec![ 
    "background", "transform", "filter", "border_top_color", "border_right_color", "border_left_color",
    "border_bottom_color", "border_top_style", "border_right_style", "border_left_style",
    "border_bottom_style", "border_top_right_radius", "border_top_left_radius",
    "border_bottom_right_radius", "border_bottom_left_radius",
  ];
  
  static ref LAYOUT_KEYS: Vec<&'static str> = vec![ 
    "flex_direction", "justify_content", "position", "align_content", "align_items", "align_self",
    "flex_wrap", "display", "overflow", "aspect_ratio", "flex_shrink", "flex_grow", "flex", "bottom",
    "end", "flex_basis", "height", "left", "margin", "margin_bottom", "margin_end", "margin_horizontal",
    "margin_left", "margin_right", "margin_start", "margin_top", "margin_vertical", "max_height", "max_width",
    "min_height", "min_width", "padding", "padding_bottom", "padding_end", "padding_horizontal", "padding_left",
    "padding_right", "padding_start", "padding_top", "padding_vertical", "right", "start", "top", "width",
    "border_bottom_width", "border_right_width", "border_left_width", "border_top_width",
  ];
}

fn get_after_match_setters(name: &str) -> TokenStream {
  let field = Ident::new(name, Span::call_site());

  quote!{
    if self.#field.0.contains_key(&property_key) {
      let mut item = self.#field.0.get_mut(&property_key).unwrap();
      *item = wrap_value;
    } else {
      self.#field.0.insert(property_key, wrap_value);
    }
  }
}

fn get_setter_apperance(field: StructField) -> TokenStream {
  let expression_setter = get_after_match_setters("apperance");
  let field_type = field.ftype;
  let name = field.name;

  quote!{
    stringify!(#name) => {
      if let Some(expected) = extract!(Apperance::#field_type(_), property) {
        let wrap_value = Apperance::#field_type(expected);
        let property_key = stringify!(#name).to_string();

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

fn get_setter_layout(field: StructField) -> TokenStream {
  // Fixes problem with `class_case` method if Inflector
  // Also check enum types for BorderWidth align
  let enum_name = field.name.to_string().to_class_case();
  let enum_name = match enum_name.as_str() {
    "BorderBottomWidth" => "BorderBottom",
    "BorderRightWidth" => "BorderRight",
    "BorderLeftWidth" => "BorderLeft",
    "BorderTopWidth" => "BorderTop",
    "AlignItem" => "AlignItems",
    "FlexBasi" => "FlexBasis",
    _ => enum_name.as_str(),
  };

  let enum_name = Ident::new(enum_name, Span::call_site());
  let expression_setter = get_after_match_setters("layout");
  let field_type = field.ftype;
  let name = field.name;

  quote!{
    stringify!(#name) => {
      if let Some(expected) = extract!(Layout::#field_type(_), property) {
        let wrap_value = FlexStyle::#enum_name(expected);
        let property_key = stringify!(#name).to_string();

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

fn get_expressions(ast_struct: DataStruct) -> (Vec<TokenStream>, Vec<TokenStream>) {
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

  let apperance_fields = collected
    .iter()
    .cloned()
    .filter(|x| APPERANCE_KEYS.contains(&x.name.to_string().as_str()))
    .map(get_setter_apperance)
    .collect::<Vec<TokenStream>>();

  let layout_fields = collected
    .iter()
    .cloned()
    .filter(|x| LAYOUT_KEYS.contains(&x.name.to_string().as_str()))
    .map(get_setter_layout)
    .collect::<Vec<TokenStream>>();

  (apperance_fields, layout_fields)
}

pub fn get_impl_trait_tokens(struct_id: Ident, data_struct: DataStruct) -> TokenStream {
  let (apperance, layout) = get_expressions(data_struct);
  let rm_matcher_apperance = match_remove_property("apperance");
  let rm_matcher_layout = match_remove_property("layout");

  quote! {
    use types::{Properties, PropertyError, PropertyValue, Apperance, Layout};
    use utils::{apperance_keys_contains, layout_keys_contains};
    use std::collections::HashMap;
    use inflector::Inflector;
    use yoga::FlexStyle;
    use traits::TStyle;

    impl TStyle for Properties {
      fn get_apperance_style(&self, name: &str) -> Option<&Apperance> {
        self.apperance.0.get(name)
      }

      fn get_layout_style(&self, name: &str) -> Option<&FlexStyle> {
        self.layout.0.get(name)
      }

      fn set_style<T: Into<Option<PropertyValue>>>(&mut self, name: &str, property: T) -> Result<(), PropertyError> {
        if apperance_keys_contains(&name) {
          self.set_apperance_style(name, extract!(PropertyValue::Apperance(_), property))
        } else if layout_keys_contains(&name) {
          self.set_layout_style(name, extract!(PropertyValue::Layout(_), property))
        } else {
          Err(PropertyError::InvalidKey { key: name.to_string() })
        }
      }

      fn set_apperance_style<T: Into<Option<Apperance>>>(&mut self, name: &str, property: T) -> Result<(), PropertyError> {
        #rm_matcher_apperance

        match name {
          #(#apperance)*
          _ => Err(PropertyError::InvalidKey {
            key: name.to_string()
          }),
        }
      }

      fn set_layout_style<T: Into<Option<Layout>>>(&mut self, name: &str, property: T) -> Result<(), PropertyError> {
        #rm_matcher_layout

        match name {
          #(#layout)*
          _ => Err(PropertyError::InvalidKey {
            key: name.to_string()
          }),
        }
      }
    }
  }
}