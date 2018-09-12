use types :: {
;
use utils
;
use std :: collections
;
;
;
use traits
;
pub mod setters {
    pub fn set_background(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::Background(_), value) {
            let wrap_value = Appearance::Background(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Background).to_string(),
                property: stringify!(background).to_string(),
            })
        }
    }
    pub fn set_transform(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::Transforms(_), value) {
            let wrap_value = Appearance::Transforms(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Transforms).to_string(),
                property: stringify!(transform).to_string(),
            })
        }
    }
    pub fn set_filter(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::Filters(_), value) {
            let wrap_value = Appearance::Filters(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Filters).to_string(),
                property: stringify!(filter).to_string(),
            })
        }
    }
    pub fn set_border_top_color(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderColor(_), value) {
            let wrap_value = Appearance::BorderColor(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderColor).to_string(),
                property: stringify!(border_top_color).to_string(),
            })
        }
    }
    pub fn set_border_right_color(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderColor(_), value) {
            let wrap_value = Appearance::BorderColor(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderColor).to_string(),
                property: stringify!(border_right_color).to_string(),
            })
        }
    }
    pub fn set_border_left_color(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderColor(_), value) {
            let wrap_value = Appearance::BorderColor(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderColor).to_string(),
                property: stringify!(border_left_color).to_string(),
            })
        }
    }
    pub fn set_border_bottom_color(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderColor(_), value) {
            let wrap_value = Appearance::BorderColor(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderColor).to_string(),
                property: stringify!(border_bottom_color).to_string(),
            })
        }
    }
    pub fn set_border_top_style(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderStyle(_), value) {
            let wrap_value = Appearance::BorderStyle(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderStyle).to_string(),
                property: stringify!(border_top_style).to_string(),
            })
        }
    }
    pub fn set_border_right_style(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderStyle(_), value) {
            let wrap_value = Appearance::BorderStyle(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderStyle).to_string(),
                property: stringify!(border_right_style).to_string(),
            })
        }
    }
    pub fn set_border_left_style(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderStyle(_), value) {
            let wrap_value = Appearance::BorderStyle(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderStyle).to_string(),
                property: stringify!(border_left_style).to_string(),
            })
        }
    }
    pub fn set_border_bottom_style(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderStyle(_), value) {
            let wrap_value = Appearance::BorderStyle(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderStyle).to_string(),
                property: stringify!(border_bottom_style).to_string(),
            })
        }
    }
    pub fn set_border_top_right_radius(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderRadius(_), value) {
            let wrap_value = Appearance::BorderRadius(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderRadius).to_string(),
                property: stringify!(border_top_right_radius).to_string(),
            })
        }
    }
    pub fn set_border_top_left_radius(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderRadius(_), value) {
            let wrap_value = Appearance::BorderRadius(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderRadius).to_string(),
                property: stringify!(border_top_left_radius).to_string(),
            })
        }
    }
    pub fn set_border_bottom_right_radius(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderRadius(_), value) {
            let wrap_value = Appearance::BorderRadius(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderRadius).to_string(),
                property: stringify!(border_bottom_right_radius).to_string(),
            })
        }
    }
    pub fn set_border_bottom_left_radius(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Appearance::BorderRadius(_), value) {
            let wrap_value = Appearance::BorderRadius(expected);
            if properties.appearance.0.contains_key(&key) {
                let mut item = properties.appearance.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.appearance.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderRadius).to_string(),
                property: stringify!(border_bottom_left_radius).to_string(),
            })
        }
    }
    pub fn set_flex_direction(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::FlexDirection(_), value) {
            let wrap_value = FlexStyle::FlexDirection(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(FlexDirection).to_string(),
                property: stringify!(flex_direction).to_string(),
            })
        }
    }
    pub fn set_justify_content(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Justify(_), value) {
            let wrap_value = FlexStyle::JustifyContent(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Justify).to_string(),
                property: stringify!(justify_content).to_string(),
            })
        }
    }
    pub fn set_position(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::PositionType(_), value) {
            let wrap_value = FlexStyle::Position(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(PositionType).to_string(),
                property: stringify!(position).to_string(),
            })
        }
    }
    pub fn set_align_content(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Align(_), value) {
            let wrap_value = FlexStyle::AlignContent(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Align).to_string(),
                property: stringify!(align_content).to_string(),
            })
        }
    }
    pub fn set_align_items(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Align(_), value) {
            let wrap_value = FlexStyle::AlignItems(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Align).to_string(),
                property: stringify!(align_items).to_string(),
            })
        }
    }
    pub fn set_align_self(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Align(_), value) {
            let wrap_value = FlexStyle::AlignSelf(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Align).to_string(),
                property: stringify!(align_self).to_string(),
            })
        }
    }
    pub fn set_flex_wrap(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Wrap(_), value) {
            let wrap_value = FlexStyle::FlexWrap(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Wrap).to_string(),
                property: stringify!(flex_wrap).to_string(),
            })
        }
    }
    pub fn set_display(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Display(_), value) {
            let wrap_value = FlexStyle::Display(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Display).to_string(),
                property: stringify!(display).to_string(),
            })
        }
    }
    pub fn set_overflow(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::Overflow(_), value) {
            let wrap_value = FlexStyle::Overflow(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(Overflow).to_string(),
                property: stringify!(overflow).to_string(),
            })
        }
    }
    pub fn set_aspect_ratio(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::AspectRatio(_), value) {
            let wrap_value = FlexStyle::AspectRatio(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(AspectRatio).to_string(),
                property: stringify!(aspect_ratio).to_string(),
            })
        }
    }
    pub fn set_flex_shrink(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::FlexShrink(_), value) {
            let wrap_value = FlexStyle::FlexShrink(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(FlexShrink).to_string(),
                property: stringify!(flex_shrink).to_string(),
            })
        }
    }
    pub fn set_flex_grow(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::FlexGrow(_), value) {
            let wrap_value = FlexStyle::FlexGrow(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(FlexGrow).to_string(),
                property: stringify!(flex_grow).to_string(),
            })
        }
    }
    pub fn set_flex(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::FlexFactor(_), value) {
            let wrap_value = FlexStyle::Flex(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(FlexFactor).to_string(),
                property: stringify!(flex).to_string(),
            })
        }
    }
    pub fn set_bottom(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Bottom(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(bottom).to_string(),
            })
        }
    }
    pub fn set_end(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::End(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(end).to_string(),
            })
        }
    }
    pub fn set_flex_basis(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::FlexBasis(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(flex_basis).to_string(),
            })
        }
    }
    pub fn set_height(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Height(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(height).to_string(),
            })
        }
    }
    pub fn set_left(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Left(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(left).to_string(),
            })
        }
    }
    pub fn set_margin(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Margin(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin).to_string(),
            })
        }
    }
    pub fn set_margin_bottom(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginBottom(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_bottom).to_string(),
            })
        }
    }
    pub fn set_margin_end(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginEnd(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_end).to_string(),
            })
        }
    }
    pub fn set_margin_horizontal(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginHorizontal(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_horizontal).to_string(),
            })
        }
    }
    pub fn set_margin_left(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginLeft(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_left).to_string(),
            })
        }
    }
    pub fn set_margin_right(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginRight(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_right).to_string(),
            })
        }
    }
    pub fn set_margin_start(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginStart(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_start).to_string(),
            })
        }
    }
    pub fn set_margin_top(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginTop(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_top).to_string(),
            })
        }
    }
    pub fn set_margin_vertical(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MarginVertical(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(margin_vertical).to_string(),
            })
        }
    }
    pub fn set_max_height(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MaxHeight(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(max_height).to_string(),
            })
        }
    }
    pub fn set_max_width(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MaxWidth(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(max_width).to_string(),
            })
        }
    }
    pub fn set_min_height(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MinHeight(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(min_height).to_string(),
            })
        }
    }
    pub fn set_min_width(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::MinWidth(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(min_width).to_string(),
            })
        }
    }
    pub fn set_padding(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Padding(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding).to_string(),
            })
        }
    }
    pub fn set_padding_bottom(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingBottom(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_bottom).to_string(),
            })
        }
    }
    pub fn set_padding_end(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingEnd(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_end).to_string(),
            })
        }
    }
    pub fn set_padding_horizontal(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingHorizontal(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_horizontal).to_string(),
            })
        }
    }
    pub fn set_padding_left(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingLeft(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_left).to_string(),
            })
        }
    }
    pub fn set_padding_right(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingRight(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_right).to_string(),
            })
        }
    }
    pub fn set_padding_start(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingStart(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_start).to_string(),
            })
        }
    }
    pub fn set_padding_top(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingTop(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_top).to_string(),
            })
        }
    }
    pub fn set_padding_vertical(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::PaddingVertical(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(padding_vertical).to_string(),
            })
        }
    }
    pub fn set_right(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Right(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(right).to_string(),
            })
        }
    }
    pub fn set_start(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Start(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(start).to_string(),
            })
        }
    }
    pub fn set_top(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Top(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(top).to_string(),
            })
        }
    }
    pub fn set_width(properties: &mut Properties, key: String, value: PropertyValue) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::StyleUnit(_), value) {
            let wrap_value = FlexStyle::Width(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(StyleUnit).to_string(),
                property: stringify!(width).to_string(),
            })
        }
    }
    pub fn set_border_bottom_width(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::BorderWidth(_), value) {
            let wrap_value = FlexStyle::BorderBottom(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderWidth).to_string(),
                property: stringify!(border_bottom_width).to_string(),
            })
        }
    }
    pub fn set_border_right_width(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::BorderWidth(_), value) {
            let wrap_value = FlexStyle::BorderRight(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderWidth).to_string(),
                property: stringify!(border_right_width).to_string(),
            })
        }
    }
    pub fn set_border_left_width(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::BorderWidth(_), value) {
            let wrap_value = FlexStyle::BorderLeft(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderWidth).to_string(),
                property: stringify!(border_left_width).to_string(),
            })
        }
    }
    pub fn set_border_top_width(
        properties: &mut Properties,
        key: String,
        value: PropertyValue,
    ) -> Result<(), PropertyError> {
        if let Some(expected) = extract!(Layout::BorderWidth(_), value) {
            let wrap_value = FlexStyle::BorderTop(expected);
            if properties.layout.0.contains_key(&key) {
                let mut item = properties.layout.0.get_mut(&key).unwrap();
                *item = wrap_value;
            } else {
                properties.layout.0.insert(key, wrap_value);
            }
            Ok(())
        } else {
            Err(PropertyError::InvalidType {
                expected: stringify!(BorderWidth).to_string(),
                property: stringify!(border_top_width).to_string(),
            })
        }
    }
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
            Err(PropertyError::InvalidKey {
                key: name.to_string(),
            })
        }
    }
    fn set_apperance_style(&mut self, name: &str, property: Option<Appearance>) -> Result<(), PropertyError> {
        if property.is_none() {
            if let Some(removed) = self.appearance.0.remove(name) {}
            return Ok(());
        }
        let property = property.unwrap();
        match name {
            stringify!(background) => setters.set_background(&mut self, name.to_string(), property),
            stringify!(transform) => setters.set_transform(&mut self, name.to_string(), property),
            stringify!(filter) => setters.set_filter(&mut self, name.to_string(), property),
            stringify!(border_top_color) => setters.set_border_top_color(&mut self, name.to_string(), property),
            stringify!(border_right_color) => setters.set_border_right_color(&mut self, name.to_string(), property),
            stringify!(border_left_color) => setters.set_border_left_color(&mut self, name.to_string(), property),
            stringify!(border_bottom_color) => setters.set_border_bottom_color(&mut self, name.to_string(), property),
            stringify!(border_top_style) => setters.set_border_top_style(&mut self, name.to_string(), property),
            stringify!(border_right_style) => setters.set_border_right_style(&mut self, name.to_string(), property),
            stringify!(border_left_style) => setters.set_border_left_style(&mut self, name.to_string(), property),
            stringify!(border_bottom_style) => setters.set_border_bottom_style(&mut self, name.to_string(), property),
            stringify!(border_top_right_radius) => {
                setters.set_border_top_right_radius(&mut self, name.to_string(), property)
            }
            stringify!(border_top_left_radius) => {
                setters.set_border_top_left_radius(&mut self, name.to_string(), property)
            }
            stringify!(border_bottom_right_radius) => {
                setters.set_border_bottom_right_radius(&mut self, name.to_string(), property)
            }
            stringify!(border_bottom_left_radius) => {
                setters.set_border_bottom_left_radius(&mut self, name.to_string(), property)
            }
            _ => Err(PropertyError::InvalidKey {
                key: name.to_string(),
            }),
        }
    }
    fn set_layout_style(&mut self, name: &str, property: Option<Layout>) -> Result<(), PropertyError> {
        if property.is_none() {
            if let Some(removed) = self.layout.0.remove(name) {}
            return Ok(());
        }
        let property = property.unwrap();
        match name {
            stringify!(flex_direction) => setters.set_flex_direction(&mut self, name.to_string(), property),
            stringify!(justify_content) => setters.set_justify_content(&mut self, name.to_string(), property),
            stringify!(position) => setters.set_position(&mut self, name.to_string(), property),
            stringify!(align_content) => setters.set_align_content(&mut self, name.to_string(), property),
            stringify!(align_items) => setters.set_align_items(&mut self, name.to_string(), property),
            stringify!(align_self) => setters.set_align_self(&mut self, name.to_string(), property),
            stringify!(flex_wrap) => setters.set_flex_wrap(&mut self, name.to_string(), property),
            stringify!(display) => setters.set_display(&mut self, name.to_string(), property),
            stringify!(overflow) => setters.set_overflow(&mut self, name.to_string(), property),
            stringify!(aspect_ratio) => setters.set_aspect_ratio(&mut self, name.to_string(), property),
            stringify!(flex_shrink) => setters.set_flex_shrink(&mut self, name.to_string(), property),
            stringify!(flex_grow) => setters.set_flex_grow(&mut self, name.to_string(), property),
            stringify!(flex) => setters.set_flex(&mut self, name.to_string(), property),
            stringify!(bottom) => setters.set_bottom(&mut self, name.to_string(), property),
            stringify!(end) => setters.set_end(&mut self, name.to_string(), property),
            stringify!(flex_basis) => setters.set_flex_basis(&mut self, name.to_string(), property),
            stringify!(height) => setters.set_height(&mut self, name.to_string(), property),
            stringify!(left) => setters.set_left(&mut self, name.to_string(), property),
            stringify!(margin) => setters.set_margin(&mut self, name.to_string(), property),
            stringify!(margin_bottom) => setters.set_margin_bottom(&mut self, name.to_string(), property),
            stringify!(margin_end) => setters.set_margin_end(&mut self, name.to_string(), property),
            stringify!(margin_horizontal) => setters.set_margin_horizontal(&mut self, name.to_string(), property),
            stringify!(margin_left) => setters.set_margin_left(&mut self, name.to_string(), property),
            stringify!(margin_right) => setters.set_margin_right(&mut self, name.to_string(), property),
            stringify!(margin_start) => setters.set_margin_start(&mut self, name.to_string(), property),
            stringify!(margin_top) => setters.set_margin_top(&mut self, name.to_string(), property),
            stringify!(margin_vertical) => setters.set_margin_vertical(&mut self, name.to_string(), property),
            stringify!(max_height) => setters.set_max_height(&mut self, name.to_string(), property),
            stringify!(max_width) => setters.set_max_width(&mut self, name.to_string(), property),
            stringify!(min_height) => setters.set_min_height(&mut self, name.to_string(), property),
            stringify!(min_width) => setters.set_min_width(&mut self, name.to_string(), property),
            stringify!(padding) => setters.set_padding(&mut self, name.to_string(), property),
            stringify!(padding_bottom) => setters.set_padding_bottom(&mut self, name.to_string(), property),
            stringify!(padding_end) => setters.set_padding_end(&mut self, name.to_string(), property),
            stringify!(padding_horizontal) => setters.set_padding_horizontal(&mut self, name.to_string(), property),
            stringify!(padding_left) => setters.set_padding_left(&mut self, name.to_string(), property),
            stringify!(padding_right) => setters.set_padding_right(&mut self, name.to_string(), property),
            stringify!(padding_start) => setters.set_padding_start(&mut self, name.to_string(), property),
            stringify!(padding_top) => setters.set_padding_top(&mut self, name.to_string(), property),
            stringify!(padding_vertical) => setters.set_padding_vertical(&mut self, name.to_string(), property),
            stringify!(right) => setters.set_right(&mut self, name.to_string(), property),
            stringify!(start) => setters.set_start(&mut self, name.to_string(), property),
            stringify!(top) => setters.set_top(&mut self, name.to_string(), property),
            stringify!(width) => setters.set_width(&mut self, name.to_string(), property),
            stringify!(border_bottom_width) => setters.set_border_bottom_width(&mut self, name.to_string(), property),
            stringify!(border_right_width) => setters.set_border_right_width(&mut self, name.to_string(), property),
            stringify!(border_left_width) => setters.set_border_left_width(&mut self, name.to_string(), property),
            stringify!(border_top_width) => setters.set_border_top_width(&mut self, name.to_string(), property),
            _ => Err(PropertyError::InvalidKey {
                key: name.to_string(),
            }),
        }
    }
}
