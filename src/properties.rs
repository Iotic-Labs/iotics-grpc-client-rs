use thiserror::Error;

use crate::{
    client::iotics::api::{LangLiteral, Literal, StringLiteral},
    common::{Property, Uri, Value},
};

pub mod common_keys {
    pub mod predicate {
        // generic
        pub const RDF_TYPE_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
        pub const LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
        pub const COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";

        // models
        pub const CREATED_FROM_PROPERTY: &str = "https://data.iotics.com/app#createdFrom";
        pub const MODEL_PROPERTY: &str = "https://data.iotics.com/app#model";

        // interactions
        pub const INTERACTION_CONFIG_PROPERTY: &str =
            "https://data.iotics.com/app#interactionConfig";

        // allow list
        pub const HOST_ALLOW_LIST_PROPERTY: &str = "http://data.iotics.com/public#hostAllowList";

        // portal
        pub const CREATED_AT_PROPERTY: &str = "https://data.iotics.com/app#createdAt";
        pub const UPDATED_AT_PROPERTY: &str = "https://data.iotics.com/app#updatedAt";
        pub const CREATED_BY_PROPERTY: &str = "https://data.iotics.com/app#createdBy";
        pub const UPDATED_BY_PROPERTY: &str = "https://data.iotics.com/app#updatedBy";
    }

    pub mod object {
        // models
        pub const BY_MODEL_PROPERTY: &str = "https://data.iotics.com/app#ByModel";

        // interactions
        pub const BY_INTERACTION_ENGINE_PROPERTY: &str =
            "https://data.iotics.com/app#ByInteractionEngine";
        pub const BY_INTERACTION_PROPERTY: &str = "https://data.iotics.com/app#ByInteraction";
        pub const INTERACTION_PROPERTY: &str = "https://data.iotics.com/app#Interaction";

        // allow list
        pub const ALL_HOST_PROPERTY: &str = "http://data.iotics.com/public#allHosts";
    }
}
pub struct PropertyBuilder;

impl PropertyBuilder {
    pub fn build_label(lang: &str, value: &str) -> Property {
        PropertyBuilder::build_lang_literal(common_keys::predicate::LABEL, lang, value)
    }

    pub fn build_lang_literal(key: &str, lang: &str, value: &str) -> Property {
        Property {
            key: key.to_string(),
            value: Some(Value::LangLiteralValue(LangLiteral {
                lang: lang.to_string(),
                value: value.to_string(),
            })),
        }
    }

    pub fn build_uri_value(key: &str, value: &str) -> Property {
        Property {
            key: key.to_string(),
            value: Some(Value::UriValue(Uri {
                value: value.to_string(),
            })),
        }
    }

    pub fn build_literal_value(key: &str, data_type: &str, value: &str) -> Property {
        Property {
            key: key.to_string(),
            value: Some(Value::LiteralValue(Literal {
                data_type: data_type.to_string(),
                value: value.to_string(),
            })),
        }
    }

    pub fn build_string_literal_value(key: &str, value: &str) -> Property {
        Property {
            key: key.to_string(),
            value: Some(Value::StringLiteralValue(StringLiteral {
                value: value.to_string(),
            })),
        }
    }
}

pub struct PropertyFinder;

impl PropertyFinder {
    pub fn find_string_literal_value<'a>(
        properties: &'a [Property],
        key: &str,
    ) -> Result<&'a str, PropertyFinderError> {
        let values = PropertyFinder::filter_values_by_key(properties, key)?;

        let value = values.into_iter().find_map(|value| match value {
            Value::StringLiteralValue(string_literal) => Some(&string_literal.value),
            _ => None,
        });

        match value {
            Some(value) => Ok(value),
            None => Err(PropertyFinderError::ValueNotFound),
        }
    }

    pub fn find_label<'a>(
        properties: &'a [Property],
        lang: &str,
    ) -> Result<&'a str, PropertyFinderError> {
        PropertyFinder::find_lang_literal_value(properties, common_keys::predicate::LABEL, lang)
    }

    pub fn find_lang_literal_value<'a>(
        properties: &'a [Property],
        key: &str,
        lang: &str,
    ) -> Result<&'a str, PropertyFinderError> {
        let values = PropertyFinder::filter_values_by_key(properties, key)?;

        let value = values.into_iter().find_map(|value| match value {
            Value::LangLiteralValue(lang_literal) => {
                if lang_literal.lang == lang {
                    Some(&lang_literal.value)
                } else {
                    None
                }
            }
            _ => None,
        });

        match value {
            Some(value) => Ok(value),
            None => Err(PropertyFinderError::LanguageNotFound),
        }
    }

    pub fn filter_values_by_key<'a>(
        properties: &'a [Property],
        key: &str,
    ) -> Result<Vec<&'a Value>, PropertyFinderError> {
        let properties = properties
            .iter()
            .filter(|property| property.key == key)
            .collect::<Vec<_>>();

        if !properties.is_empty() {
            let values = properties
                .iter()
                .filter_map(|property| {
                    if let Some(value) = &property.value {
                        Some(value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Value>>();

            if !values.is_empty() {
                Ok(values)
            } else {
                Err(PropertyFinderError::ValueNotFound)
            }
        } else {
            Err(PropertyFinderError::KeyNotFound)
        }
    }
}

#[derive(Error, Debug)]
pub enum PropertyFinderError {
    #[error("property key not found")]
    KeyNotFound,
    #[error("property value not found")]
    ValueNotFound,
    #[error("language not found")]
    LanguageNotFound,
}
