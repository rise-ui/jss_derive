use common::{StructField, optioned_type, field_to_name_and_ty };
use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use regex::Regex;

lazy_static! {
  static ref BORDER_PROPERTY: Regex = Regex::new(r"^border_(?P<edge>\w+)_(radius|width|style|color)$").unwrap();
}

fn generate_style_property(field: StructField) -> TokenStream {
  let ftype = field.ftype;
  let name = field.name;

  quote!{
    stringify!(#name) => {
      let value: #ftype = from_value(value)?;
      if let Some (ref mut styles) = style.default {
        styles.#name = Some(value);
      }
    },
  }
}

fn generate_style_edge_property(field: (&str, &str)) -> TokenStream {
  let match_name = Ident::new(&*field.0.replace("_*", ""), Span::call_site());
  let ftype = Ident::new(field.1, Span::call_site());

  let edges = vec!["top", "right", "bottom", "left"].into_iter().map(|edge| {
    let field = Ident::new(&*field.0.replace("*", edge), Span::call_site());
    quote!{ styles.#field = Some(value); }
  });

  quote!{
    stringify!(#match_name) => {
      let value: #ftype = from_value(value)?;
      if let Some (ref mut styles) = style.default {
        #(#edges)*
      }
    },
  }
}

fn generate_style_corner_property(field: (&str, &str)) -> TokenStream {
  let match_name = Ident::new(&*field.0.replace("_*", ""), Span::call_site());
  let ftype = Ident::new(field.1, Span::call_site());

  let corners = vec!["top_left", "top_right", "bottom_left", "bottom_right"].into_iter().map(|corner| {
    let field = Ident::new(&*field.0.replace("*", corner), Span::call_site());
    quote!{ styles.#field = Some(value); }
  });

  quote!{
    stringify!(#match_name) => {
      let value: #ftype = from_value(value)?;
      if let Some (ref mut styles) = style.default {
        #(#corners)*
      }
    },
  }
}

fn generate_static_property(field: StructField) -> TokenStream {
  let name = field.name;
  let name = quote!{ stringify!(#name) };

  quote!{ properties.push(#name); }
}

fn get_collect_expressions(
  ast_struct: DataStruct,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
  let fields = if let Fields::Named(x) = ast_struct.fields {
    x.named
  } else {
    panic!("Only structs with named fields are supported.");
  };

  let uniform_corder = vec![("border_*_radius", "BorderRadius")];
  let uniform_edge =
    vec![("border_*_style", "BorderStyle"), ("border_*_color", "BorderColor"), ("border_*_width", "BorderWidth")];

  let collected = fields
    .into_iter()
    .filter_map(field_to_name_and_ty)
    .filter_map(|(name, type_path)| optioned_type(name, type_path.segments))
    .collect::<Vec<StructField>>();

  let corners: Vec<TokenStream> = uniform_corder.iter().cloned().map(generate_style_corner_property).collect();
  let edges: Vec<TokenStream> = uniform_edge.iter().cloned().map(generate_style_edge_property).collect();
  let matches: Vec<TokenStream> = collected.iter().cloned().map(generate_style_property).collect();
  let keys: Vec<TokenStream> = collected.iter().cloned().map(generate_static_property).collect();

  (matches, corners, edges, keys)
}

fn get_parse_expression(
  target: &str,
  expressions: (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>),
) -> TokenStream {
  let target = Ident::new(target, Span::call_site());
  let (matches, corners, edges, _) = expressions;

  quote! {
    use #target::{Value, from_value};
    let parsed: HashMap<String, Value> = #target::from_str(source)?;
    let mut style = Style::default();

    style.default = Some(StyleProperties::default());

    for (key, value) in parsed.into_iter() {
      let field_key = &*key.to_snake_case();

      let found_in_properties = STYLE_PROPERTIES.iter()
        .position(|&p| p == field_key)
        .is_some();

      if found_in_properties {
        let is_valid_case = match case_style {
          PropertyCase::Snake => key.is_snake_case(),
          PropertyCase::Kebab => key.is_kebab_case(),
          PropertyCase::Camel => key.is_camel_case(),
        };

        if is_valid_case {
          match field_key {
            #(#matches)*
            #(#corners)*
            #(#edges)*
            _ => {}
          }
        } else {
          bail!(ParseError::InvalidKeyCase {
            case: case_style,
            key,
          })
        }
      } else {
        // @todo: handle @media properties or other macros
      }
    }
    Ok(style)
  }
}

pub fn get_impl_trait_tokens(struct_id: Ident, data_struct: DataStruct) -> TokenStream {
  let expressions = get_collect_expressions(data_struct);

  let json_tokens = get_parse_expression("serde_json", expressions.clone());
  let yaml_tokens = get_parse_expression("serde_yaml", expressions.clone());

  let keys = expressions.3;

  quote! {
    use std::collections::HashMap;
    use inflector::Inflector;
    use failure::Error;
    use serde_json;
    use serde_yaml;

    use common::{
      RecursiveType,
      ParseOptions,
      PropertyCase,
      ParseTarget,
      ParseError,
      Style,
    };

    lazy_static!{
      static ref STYLE_PROPERTIES: Vec<&'static str> = {
        let mut properties = Vec::new();
        #(#keys)*
        properties
      };
    }

    impl ParseStyleExt for Style {
      fn parse_json_element(source: &str, recursive: RecursiveType, case_style: PropertyCase) -> Result<Style, Error> {
        #json_tokens
      }

      fn parse_yaml_element(source: &str, recursive: RecursiveType, case_style: PropertyCase) -> Result<Style, Error> {
        #yaml_tokens
      }

      fn parse_element(source: &str, options: ParseOptions) -> Result<Style, Error> {
        match options.from {
          ParseTarget::Json => Style::parse_json_element(source, options.recursive, options.style),
          ParseTarget::Yaml => Style::parse_yaml_element(source, options.recursive, options.style),
        }
      }
    }
  }
}
