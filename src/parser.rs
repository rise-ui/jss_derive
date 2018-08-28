use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;

use common::{
  apperance_keys_contains,
  layout_keys_contains,
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

  let is_apperance = apperance_keys_contains(&name.to_string());

  let action_method: Ident = if is_apperance {
    Ident::new("set_apperance_style", Span::call_site())
  } else {
    Ident::new("set_layout_style", Span::call_site())
  };

  let wrapper: Ident = if is_apperance {
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
      _ => source.as_str()
    };

    Ident::new(source, Span::call_site()) 
  };

  (wrapper, wrapper_ty, action_method)
}

fn get_match_expression(field: StructField) -> TokenStream {
  let field_ty = field.ftype.clone();
  let name = field.name.clone();

  let (wrapper, wrapper_ty, action_method) = get_expression_params(field);

  quote!{
    stringify!(#name) => {
      use self::#wrapper::#wrapper_ty as Content;

      let parsed: Result<#field_ty, ParseError> = from_value(value)
        .map_err(|error| ConversationError {
          property: key.clone(),
          error
        });
      
      match parsed {
        Err(error) => warnings.push(error),

        Ok(value) => {
          let inserted = properties_default
            .#action_method(key.as_str(), Some(Content(value)))
            .map_err(|error| ErrorPasteProperty {
              property: key.clone(),
              error
            });
          
          if let Err(error) = inserted {
            warnings.push(error);
          }
        },
      }
    },
  }
}

fn get_parse_expression(target: &str, matches: Vec<TokenStream>) -> TokenStream {
  let imports = Ident::new(format!("serde_{}", target).as_str(), Span::call_site());
  
  let error_parse = format!("Invalid{}", target.to_uppercase());
  let error_deser = format!("{}Value", error_parse);

  let deser_error = Ident::new(error_deser.as_str(), Span::call_site());
  let parse_error = Ident::new(error_parse.as_str(), Span::call_site());

  quote!{
    use #imports::{Value, from_value, from_str, Error as SourceParseError };

    use self::ParseError::{InvalidKeyCase, ErrorPasteProperty};
    use self::ParseError::#deser_error as ConversationError;    
    use self::ParseError::#parse_error as InvalidSourceError;

    let parsed: HashMap<String, Value> = from_str(source)
      .map_err(|error| InvalidSourceError { error })?;
    
    let mut properties_default = Properties::default();
    let mut style = Style::default();
    let mut warnings = vec![];

    for (key, value) in parsed {
      let action_prefix = &key[..1];          
      match action_prefix {
        // States like :hover, :active
        ":" => {}
        // Nested insides
        "&" => {},
        // actions handlers
        "@" => {},
        // Otherwise standard properties
        _ => {
          if is_valid_case(&key, case_style) {
            // After check convert key to snake_case
            let key = key.to_snake_case();

            match key.as_str() {
              #(#matches)*

              // Otherwise push error to warnings
              _ => {
                warnings.push(ErrorPasteProperty {
                  error: PropertyError::InvalidKey { key: key.clone() },
                  property: key.clone(),
                });
              }
            }
          }          
        }
      }
    }

    // Insert default properties to default state
    style.states.insert("default".to_string(), properties_default);

    Ok(ParseResult { warnings, style })
  }
} 

pub fn get_impl_trait_tokens(struct_id: Ident, data_struct: DataStruct) -> TokenStream {
  let matches = get_expressions(data_struct);

  let json = get_parse_expression("json", matches.clone());
  let yaml = get_parse_expression("yaml", matches.clone());

  quote! {
    use types::{Style, ParseError};
    use utils::is_valid_case;
    use serde_json;
    use serde_yaml;

    use types::property_types::*;
    use traits::*;
    
    use types::parser::{
      RecursiveType,
      ParseOptions,
      PropertyCase,
      ParseTarget,
      ParseResult,
    };

    impl TParseStyle for Style {
      // Uniform function for parse element
      fn parse_element(source: &str, options: ParseOptions) -> Result<ParseResult, ParseError> {
        match options.from {
          ParseTarget::Json => Style::parse_json_element(source, options.recursive, options.style),
          ParseTarget::Yaml => Style::parse_yaml_element(source, options.recursive, options.style),
        }
      }
      
      // Parse element on JSON
      fn parse_json_element(source: &str, recursive: RecursiveType, case_style: PropertyCase) -> Result<ParseResult, ParseError> {
        #json
      }

      // Parse element on YAML
      fn parse_yaml_element(source: &str, recursive: RecursiveType, case_style: PropertyCase) -> Result<ParseResult, ParseError> {
        #yaml        
      }
    }
  }
}