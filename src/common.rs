use syn::{GenericArgument, Field, Path, PathArguments, PathSegment, Type};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;

lazy_static! {
    static ref APPERANCE_KEYS: Vec<&'static str> = vec![
        "background",
        "transform",
        "filter",
        "border_top_color",
        "border_right_color",
        "border_left_color",
        "border_bottom_color",
        "border_top_style",
        "border_right_style",
        "border_left_style",
        "border_bottom_style",
        "border_top_right_radius",
        "border_top_left_radius",
        "border_bottom_right_radius",
        "border_bottom_left_radius",
    ];
    static ref LAYOUT_KEYS: Vec<&'static str> = vec![
        "flex_direction",
        "justify_content",
        "position",
        "align_content",
        "align_items",
        "align_self",
        "flex_wrap",
        "display",
        "overflow",
        "aspect_ratio",
        "flex_shrink",
        "flex_grow",
        "flex",
        "bottom",
        "end",
        "flex_basis",
        "height",
        "left",
        "margin",
        "margin_bottom",
        "margin_end",
        "margin_horizontal",
        "margin_left",
        "margin_right",
        "margin_start",
        "margin_top",
        "margin_vertical",
        "max_height",
        "max_width",
        "min_height",
        "min_width",
        "padding",
        "padding_bottom",
        "padding_end",
        "padding_horizontal",
        "padding_left",
        "padding_right",
        "padding_start",
        "padding_top",
        "padding_vertical",
        "right",
        "start",
        "top",
        "width",
        "border_bottom_width",
        "border_right_width",
        "border_left_width",
        "border_top_width",
    ];
}

pub fn apperance_keys_contains(name: &str) -> bool {
    APPERANCE_KEYS.contains(&name)
}

pub fn layout_keys_contains(name: &str) -> bool {
    LAYOUT_KEYS.contains(&name)
}

/// A field description.
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: Ident,
    /// Underlying type of the original field (if the field was `Option<T>`, this would be `T).
    pub ftype: Ident,
}

/// Checks whether a given field is an `Option<T>` and returns it as a field description on success.
pub fn optioned_type<T>(field_name: Ident, segments: Punctuated<PathSegment, T>) -> Option<StructField> {
    if segments.len() != 1 {
        return None;
    }
    let ty = segments.into_iter().next().unwrap();
    if ty.ident != "Option" {
        return None;
    }
    extract!(PathArguments::AngleBracketed(_), ty.arguments)
        .and_then(|generics| generics.args.into_iter().next())
        .and_then(|arg| extract!(GenericArgument::Type(_), arg))
        .and_then(|ty| extract!(Type::Path(_), ty))
        .and_then(|ty_path| ty_path.path.segments.into_iter().next())
        .map(|segment| segment.ident)
        .map(|ftype| StructField {
            name: field_name,
            ftype,
        })
}

/// Checks if a named field's type is TypePath and returns its name and the path on success.
pub fn field_to_name_and_ty(field: Field) -> Option<(Ident, Path)> {
    if let Type::Path(ty) = field.ty {
        field.ident.map(|id| (id, ty.path))
    } else {
        None
    }
}
